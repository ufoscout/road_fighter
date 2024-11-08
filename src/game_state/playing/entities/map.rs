use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{constants::WINDOW_HEIGHT, game_state::{playing::{assets::{colliders, AssetKey}, CollidedWithWall, MapData, MapTile, PlayerOneCar, PlayingAll, PlayingData, PlayingMap}, GameGlobalState}};

use super::GameLayer;

/// The plugin that handles the map
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
                Update,
                (
                    render_screen, 
                    wall_collisions,
            ).run_if(in_state(GameGlobalState::Playing)),
        );
    }
}

/// Spans the map on the screen and returns the minimum y coordinate of the map
pub fn span_map(playing_data: &PlayingData,
    commands: &mut Commands, asset_server: &AssetServer, texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,) -> f32 {
    
    // Load the map data
    let map_data = playing_data.level.map_data();
    let mut min_y = 0f32;
    
    // Spawn background tiles
    for map_tile in map_data.background_tiles.iter() {
        let y = span_map_tile(commands, &asset_server, texture_atlas_layouts, map_tile, map_data, 0.);
        min_y = min_y.min(y);
    }
    
    // Spawn middleground tiles
    for map_tile in map_data.middleground_tiles.iter() {
        let y = span_map_tile(commands, &asset_server, texture_atlas_layouts, map_tile, map_data, 1.);
        min_y = min_y.min(y);
    }
    
    // // Spawn foreground tiles
    for map_tile in map_data.foreground_tiles.iter() {
        let y = span_map_tile(commands, &asset_server, texture_atlas_layouts, map_tile, map_data, 2.);
        min_y = min_y.min(y);
    }

    min_y += WINDOW_HEIGHT / 2.0;
    min_y
    
}

/// Spawns a tile on the screen and return the entity y coordinate
fn span_map_tile(commands: &mut Commands, asset_server: &AssetServer, texture_atlas_layouts: &mut Assets<TextureAtlasLayout>, map_tile: &MapTile, map_data: &MapData, z: f32) -> f32 {
    let tile_data = map_data.tiles.get(map_tile.tile_bank)
        .and_then(|t| t.get(map_tile.tile_num)).expect(&format!("cannot find tile bank {}, num {}", map_tile.tile_bank, map_tile.tile_num));


    // Calculate the position of the tile
    // The original position is the top left corner of the tile, but bevy uses the center of the sprite
    let half_screen_x = WINDOW_HEIGHT / 2.0;
    let half_screen_y = WINDOW_HEIGHT / 2.0;
    let x = map_tile.x - half_screen_x + tile_data.width as f32 / 2.0;
    let y = half_screen_y - map_tile.y - tile_data.height as f32 / 2.0;

    if let Some(collider) = colliders().get(&AssetKey {
        tile_source: &tile_data.tile_source,
        x: tile_data.x as u32,
        y: tile_data.y as u32,
    }) {
        // Spawn the object
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&tile_data.tile_source),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(tile_data.width, tile_data.height),
                    1,
                    1,
                    None,
                    Some(UVec2::new(tile_data.x as u32, tile_data.y as u32)),
                )),
            },
            PlayingAll,
            PlayingMap {
                y_position: y,
            },
            collider.clone(),
            RigidBody::Static,
            CollisionLayers::new(GameLayer::Wall, [GameLayer::Player]),
            // DebugRender::default().with_collider_color(Color::WHITE),
        ));
    } else {
        // Spawn the object
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(&tile_data.tile_source),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(tile_data.width, tile_data.height),
                    1,
                    1,
                    None,
                    Some(UVec2::new(tile_data.x as u32, tile_data.y as u32)),
                )),
            },
            PlayingAll,
            PlayingMap {
                y_position: y,
            },
        ));
    }

    y
}

pub fn wall_collisions(
    mut commands: Commands,
    map: Query<(&PlayingMap, &CollidingEntities)>,
) {
    for (_map, collisions) in map.iter() {
        collisions.iter().for_each(|entity| {
            // println!("Map collision with entity {:?}", entity);
            commands.entity(*entity).insert(CollidedWithWall);
        });
    }
}

pub fn render_screen(
    car: Query<&PlayerOneCar>,
    mut map_data: Query<(&PlayingMap, &mut Transform), Without<PlayerOneCar>>,
) {

    let car = car.single();

    for (map, mut transform) in map_data.iter_mut() {
        transform.translation.y = map.y_position - car.y_position;
    }
    
}