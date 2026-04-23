use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    chunk_manager::{ChunkManagerConfig, ChunkScroll, TileBlit},
    constants::WINDOW_HEIGHT,
    game_state::{
        playing::{
            assets::{add_collider, AssetKey},
            constants::MAP_ORIGIN_X,
            CarCollidedSide, CollidedWithWall, LeftWall, MapData, MapTile, PlayerOneCar,
            PlayingAll, PlayingData, PlayingMap, RightWall,
        },
        GameGlobalState,
    },
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_chunk_scroll,
                render_collision_tiles,
                left_wall_collisions,
                right_wall_collisions,
            )
                .run_if(in_state(GameGlobalState::Playing)),
        );
    }
}

/// Builds the [`ChunkManagerConfig`] for the given map and returns the initial
/// scroll value (world-Y of the bottommost tile, used to spawn the player car).
pub fn build_map_config(playing_data: &PlayingData) -> (ChunkManagerConfig, f32) {
    let map_data = playing_data.level.map_data();

    let mut tiles: Vec<TileBlit> = Vec::new();
    collect_blits(&mut tiles, &map_data.background_tiles, map_data, 0);
    collect_blits(&mut tiles, &map_data.middleground_tiles, map_data, 1);
    collect_blits(&mut tiles, &map_data.foreground_tiles, map_data, 2);

    // map_origin: world-space position of map pixel (0, 0).
    //   x = tile.map_x - WINDOW_WIDTH/2  + tile.width/2
    //   y = WINDOW_HEIGHT/2 - tile.map_y - tile.height/2
    let map_origin = Vec2::new(MAP_ORIGIN_X, WINDOW_HEIGHT / 2.0);

    let config = ChunkManagerConfig {
        map_width: map_data.width as u32,
        map_height: map_data.height,
        chunk_height: 3840,
        map_origin,
        assets_base_path: "assets/".to_string(),
        tiles,
    };

    let initial_scroll = compute_initial_scroll(map_data);
    (config, initial_scroll)
}

/// Spawns invisible collision entities for road-border tiles. No Sprite component
/// is needed — the visual is handled by the chunk manager.
pub fn spawn_collision_tiles(
    playing_data: &PlayingData,
    commands: &mut Commands,
) {
    let map_data = playing_data.level.map_data();
    // Road borders are always in the middleground layer.
    for map_tile in map_data.middleground_tiles.iter() {
        let tile_data = map_data
            .tiles
            .get(map_tile.tile_bank)
            .and_then(|b| b.get(map_tile.tile_num))
            .expect("tile lookup failed");

        let asset_key = AssetKey {
            tile_source: &tile_data.tile_source,
            x: tile_data.x as u32,
            y: tile_data.y as u32,
        };

        // Only spawn an entity if this tile has a collider.
        if !add_collider_check(&asset_key) {
            continue;
        }

        let x = MAP_ORIGIN_X + map_tile.x + tile_data.width as f32 / 2.0;
        let y = WINDOW_HEIGHT / 2.0 - map_tile.y - tile_data.height as f32 / 2.0;

        let mut entity = commands.spawn((
            Transform::from_translation(Vec3::new(x, y, 1.0)),
            PlayingAll,
            PlayingMap { y_position: y },
        ));
        add_collider(&mut entity, asset_key);
    }
}

// ─── Systems ─────────────────────────────────────────────────────────────────

fn update_chunk_scroll(
    car: Query<&PlayerOneCar>,
    mut scroll: ResMut<ChunkScroll>,
) {
    if let Ok(car) = car.single() {
        scroll.y = car.y_position.round();
    }
}

/// Scrolls the small number of collision-only entities (no sprites).
fn render_collision_tiles(
    scroll: Res<ChunkScroll>,
    mut query: Query<(&PlayingMap, &mut Transform), Without<PlayerOneCar>>,
) {
    let y = scroll.y;
    for (map, mut transform) in query.iter_mut() {
        transform.translation.y = map.y_position - y;
    }
}

pub fn left_wall_collisions(
    mut commands: Commands,
    map: Query<(&LeftWall, &CollidingEntities)>,
) {
    for (_wall, collisions) in map.iter() {
        collisions.iter().for_each(|entity| {
            commands.get_entity(*entity).map(|mut e| {
                e.try_insert(CollidedWithWall { side: CarCollidedSide::Left });
            });
        });
    }
}

pub fn right_wall_collisions(
    mut commands: Commands,
    map: Query<(&RightWall, &CollidingEntities)>,
) {
    for (_wall, collisions) in map.iter() {
        collisions.iter().for_each(|entity| {
            commands.get_entity(*entity).map(|mut e| {
                e.try_insert(CollidedWithWall { side: CarCollidedSide::Right });
            });
        });
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn collect_blits(
    out: &mut Vec<TileBlit>,
    map_tiles: &[MapTile],
    map_data: &MapData,
    z_order: u8,
) {
    for map_tile in map_tiles {
        let tile_data = map_data
            .tiles
            .get(map_tile.tile_bank)
            .and_then(|b| b.get(map_tile.tile_num))
            .expect("tile lookup failed");

        out.push(TileBlit {
            image_path: tile_data.tile_source.clone(),
            src_x: tile_data.x,
            src_y: tile_data.y,
            width: tile_data.width,
            height: tile_data.height,
            map_x: map_tile.x,
            map_y: map_tile.y,
            z_order,
        });
    }
}

fn compute_initial_scroll(map_data: &MapData) -> f32 {
    let half_screen = WINDOW_HEIGHT / 2.0;
    let min_y = map_data
        .background_tiles
        .iter()
        .chain(map_data.middleground_tiles.iter())
        .chain(map_data.foreground_tiles.iter())
        .map(|t| {
            let tile = &map_data.tiles[t.tile_bank][t.tile_num];
            half_screen - t.y - tile.height as f32 / 2.0
        })
        .fold(f32::INFINITY, f32::min);
    min_y + half_screen
}

/// Returns true if this tile key would receive a collider from [`add_collider`].
/// Mirrors the match arms in assets.rs to avoid spawning entities for non-collision tiles.
fn add_collider_check(key: &AssetKey) -> bool {
    matches!(
        key,
        AssetKey { tile_source: "graphics/road.png", x: 0,   y: 128 }
        | AssetKey { tile_source: "graphics/road.png", x: 0,   y: 256 }
        | AssetKey { tile_source: "graphics/road.png", x: 96,  y: 128 }
        | AssetKey { tile_source: "graphics/road.png", x: 96,  y: 256 }
        | AssetKey { tile_source: "graphics/road.png", x: 192, y: 128 }
        | AssetKey { tile_source: "graphics/road.png", x: 192, y: 256 }
        | AssetKey { tile_source: "graphics/road.png", x: 288, y: 128 }
        | AssetKey { tile_source: "graphics/road.png", x: 288, y: 256 }
        | AssetKey { tile_source: "graphics/road.png", x: 400, y: 128 }
        | AssetKey { tile_source: "graphics/road.png", x: 400, y: 256 }
    )
}
