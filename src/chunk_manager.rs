//! A generic GPU-chunked map renderer for large vertically-scrolling tile maps.
//! The manager keeps 4 chunk textures alive at a time (1 behind the camera,
//! 3 ahead). Each chunk is composited on a background thread.
//! 
//! We cannot use existing solutions like [`bevy_ecs_tilemap`](https://crates.io/crates/bevy_ecs_tilemap)
//! because it requires all sprites to be of the same size which is not the case here.

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    tasks::{futures_lite::future, AsyncComputeTaskPool, Task},
};
use bevy::asset::RenderAssetUsages;
use image::RgbaImage;


/// One tile blit instruction.
#[derive(Clone)]
pub struct TileBlit {
    /// Path relative to [`ChunkManagerConfig::assets_base_path`], e.g. `"graphics/road.png"`.
    pub image_path: String,
    /// X offset of the source rectangle inside the source image (pixels).
    pub src_x: u32,
    /// Y offset of the source rectangle inside the source image (pixels).
    pub src_y: u32,
    pub width: u32,
    pub height: u32,
    /// Top-left X of this tile in map pixel space.
    pub map_x: f32,
    /// Top-left Y of this tile in map pixel space (Y = 0 is the top).
    pub map_y: f32,
    /// Paint order within a chunk — lower values are drawn first.
    pub z_order: u8,
}

/// Configuration for the [`ChunkManager`].
pub struct ChunkManagerConfig {
    /// Width of the whole map in pixels.
    pub map_width: u32,
    /// Height of the whole map in pixels.
    pub map_height: f32,
    /// Height of each chunk in map pixels. Recommended: 8–16× the viewport height.
    pub chunk_height: u32,
    /// World-space XY corresponding to map pixel (0, 0) — the top-left corner of the
    /// map at `scroll_y = 0`. Bevy world-space Y increases upward; map-space Y
    /// increases downward.
    pub map_origin: Vec2,
    /// Filesystem prefix prepended to every [`TileBlit::image_path`], e.g. `"assets/"`.
    pub assets_base_path: String,
    /// All tile blit instructions for the entire map (all layers combined).
    pub tiles: Vec<TileBlit>,
}

/// To be updated every frame.
/// `y` is the same scroll value used to offset entity transforms:
/// `transform.translation.y = base_world_y - scroll.y`.
#[derive(Resource, Default)]
pub struct ChunkScroll {
    pub y: f32,
}

/// Marker component on every chunk sprite entity spawned by the manager.
#[derive(Component)]
pub struct ChunkSprite {
    chunk_index: usize,
    base_world_y: f32,
}

#[derive(Resource)]
pub(crate) struct ChunkManager {
    config: ChunkManagerConfig,
    total_chunks: usize,
    /// Per-chunk sorted tile index lists, pre-computed at setup.
    tiles_by_chunk: Vec<Vec<usize>>,
    source_images: Arc<HashMap<String, RgbaImage>>,
    /// chunk_index → spawned Entity
    active: HashMap<usize, Entity>,
    /// Background compositing tasks: (chunk_index, task)
    pending: Vec<(usize, Task<(usize, Vec<u8>)>)>,
}


pub struct ChunkManagerPlugin;

impl Plugin for ChunkManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkScroll>().add_systems(
            Update,
            (poll_pending_tasks, update_active_chunks, scroll_chunk_sprites)
                .chain()
                .run_if(resource_exists::<ChunkManager>),
        );
    }
}


/// Loads all unique source images from disk synchronously,
/// pre-computes per-chunk tile lists, inserts [`ChunkScroll`] initialised to
/// `initial_scroll`, and kicks off background compositing for the first visible
/// chunks.
/// Call once from the level-enter system.
pub fn setup_chunk_manager(
    config: ChunkManagerConfig,
    initial_scroll: f32,
    commands: &mut Commands,
) {
    // Load every unique source image once.
    let unique_paths: HashSet<&String> = config.tiles.iter().map(|t| &t.image_path).collect();
    let mut source_map: HashMap<String, RgbaImage> = HashMap::new();
    for path in unique_paths {
        let full = format!("{}{}", config.assets_base_path, path);
        let img = image::open(&full)
            .unwrap_or_else(|e| panic!("ChunkManager: cannot open {full}: {e}"))
            .to_rgba8();
        source_map.insert(path.clone(), img);
    }

    let total_chunks = (config.map_height / config.chunk_height as f32).ceil() as usize;

    // Assign tiles to chunks. Tiles that span a boundary appear in both chunks.
    let mut tiles_by_chunk: Vec<Vec<usize>> = vec![Vec::new(); total_chunks];
    for (i, tile) in config.tiles.iter().enumerate() {
        let first = (tile.map_y / config.chunk_height as f32).floor() as usize;
        let last = ((tile.map_y + tile.height as f32 - 1.0) / config.chunk_height as f32)
            .floor() as usize;
        for ci in first..=last.min(total_chunks.saturating_sub(1)) {
            tiles_by_chunk[ci].push(i);
        }
    }
    for bucket in &mut tiles_by_chunk {
        bucket.sort_by_key(|&i| config.tiles[i].z_order);
    }

    let source_images = Arc::new(source_map);

    // Determine which chunks are initially visible.
    let chunk_height = config.chunk_height as f32;
    let map_origin_y = config.map_origin.y;
    let viewport_map_y = map_origin_y - initial_scroll;
    let center = (viewport_map_y / chunk_height).floor() as i64;

    let mut manager = ChunkManager {
        total_chunks,
        tiles_by_chunk,
        source_images,
        active: HashMap::new(),
        pending: Vec::new(),
        config,
    };

    // Pre-load 4 chunks around the starting position.
    for d in -1_i64..=2 {
        let ci = center + d;
        if ci >= 0 && (ci as usize) < total_chunks {
            enqueue_chunk(&mut manager, ci as usize);
        }
    }

    commands.insert_resource(ChunkScroll { y: initial_scroll });
    commands.insert_resource(manager);
}

/// Call from the level-exit system to despawn all chunk sprites and remove the
/// manager resource.
pub fn cleanup_chunk_manager(
    mut commands: Commands,
    manager: Option<ResMut<ChunkManager>>,
    sprites: Query<Entity, With<ChunkSprite>>,
) {
    if manager.is_some() {
        commands.remove_resource::<ChunkManager>();
    }
    for entity in &sprites {
        commands.entity(entity).despawn();
    }
}


fn enqueue_chunk(manager: &mut ChunkManager, chunk_index: usize) {
    if manager.active.contains_key(&chunk_index) {
        return;
    }
    if manager.pending.iter().any(|(ci, _)| *ci == chunk_index) {
        return;
    }

    let chunk_height = manager.config.chunk_height;
    let map_width = manager.config.map_width;
    let tiles: Vec<TileBlit> = manager.tiles_by_chunk[chunk_index]
        .iter()
        .map(|&i| manager.config.tiles[i].clone())
        .collect();
    let sources = manager.source_images.clone();

    let task = AsyncComputeTaskPool::get().spawn(async move {
        composite_chunk(chunk_index, map_width, chunk_height, &tiles, &sources)
    });
    manager.pending.push((chunk_index, task));
}

fn composite_chunk(
    chunk_index: usize,
    map_width: u32,
    chunk_height: u32,
    tiles: &[TileBlit],
    sources: &HashMap<String, RgbaImage>,
) -> (usize, Vec<u8>) {
    let y_start = chunk_index as f32 * chunk_height as f32;
    let mut canvas = RgbaImage::new(map_width, chunk_height);

    for tile in tiles {
        let src = match sources.get(&tile.image_path) {
            Some(img) => img,
            None => continue,
        };
        let dest_x = tile.map_x as i64;
        let dest_y = (tile.map_y - y_start) as i64;

        for py in 0..tile.height {
            let sy = tile.src_y + py;
            if sy >= src.height() {
                continue;
            }
            let dy = dest_y + py as i64;
            if dy < 0 || dy >= chunk_height as i64 {
                continue;
            }
            for px in 0..tile.width {
                let sx = tile.src_x + px;
                if sx >= src.width() {
                    continue;
                }
                let pixel = *src.get_pixel(sx, sy);
                if pixel[3] == 0 {
                    continue;
                }
                let dx = dest_x + px as i64;
                if dx < 0 || dx >= map_width as i64 {
                    continue;
                }
                canvas.put_pixel(dx as u32, dy as u32, pixel);
            }
        }
    }

    (chunk_index, canvas.into_raw())
}

fn poll_pending_tasks(
    mut manager: ResMut<ChunkManager>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    let chunk_width = manager.config.map_width;
    let chunk_height = manager.config.chunk_height;
    let map_origin = manager.config.map_origin;

    let mut completed: Vec<(usize, Vec<u8>)> = Vec::new();
    manager.pending.retain_mut(|(_, task)| {
        match future::block_on(future::poll_once(task)) {
            None => true,
            Some(result) => {
                completed.push(result);
                false
            }
        }
    });

    for (chunk_index, bytes) in completed {
        let bevy_image = Image::new(
            Extent3d { width: chunk_width, height: chunk_height, depth_or_array_layers: 1 },
            TextureDimension::D2,
            bytes,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );
        let handle = images.add(bevy_image);

        let base_world_y = map_origin.y
            - chunk_index as f32 * chunk_height as f32
            - chunk_height as f32 / 2.0;
        let world_x = map_origin.x + chunk_width as f32 / 2.0;

        let entity = commands
            .spawn((
                Sprite { image: handle, ..default() },
                Transform::from_translation(Vec3::new(world_x, base_world_y, 0.0)),
                ChunkSprite { chunk_index, base_world_y },
            ))
            .id();
        manager.active.insert(chunk_index, entity);
    }
}

fn update_active_chunks(
    scroll: Res<ChunkScroll>,
    mut manager: ResMut<ChunkManager>,
    mut commands: Commands,
) {
    let chunk_height = manager.config.chunk_height as f32;
    let map_origin_y = manager.config.map_origin.y;
    let total = manager.total_chunks;

    let viewport_map_y = map_origin_y - scroll.y;
    let center = (viewport_map_y / chunk_height).floor() as i64;

    // Keep 1 chunk behind the camera, 3 chunks ahead.
    let desired: HashSet<usize> = (-1_i64..=3)
        .filter_map(|d| {
            let ci = center + d;
            if ci >= 0 && (ci as usize) < total { Some(ci as usize) } else { None }
        })
        .collect();

    // Despawn stale active chunks.
    let stale: Vec<usize> =
        manager.active.keys().filter(|ci| !desired.contains(ci)).copied().collect();
    for ci in stale {
        if let Some(entity) = manager.active.remove(&ci) {
            commands.entity(entity).despawn();
        }
    }
    // Drop stale pending tasks (they will be cancelled when the Task is dropped).
    manager.pending.retain(|(ci, _)| desired.contains(ci));

    // Enqueue newly required chunks.
    let missing: Vec<usize> = desired
        .into_iter()
        .filter(|ci| !manager.active.contains_key(ci))
        .collect();
    for ci in missing {
        enqueue_chunk(&mut manager, ci);
    }
}

fn scroll_chunk_sprites(
    scroll: Res<ChunkScroll>,
    mut query: Query<(&ChunkSprite, &mut Transform)>,
) {
    let y = scroll.y;
    for (chunk, mut transform) in &mut query {
        transform.translation.y = chunk.base_world_y - y;
    }
}
