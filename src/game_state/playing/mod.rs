use avian2d::prelude::*;
use bevy::prelude::*;
use components::*;
use entities::*;
use explosion::PlayerCarExplosionPlugin;
use map::{build_map_config, spawn_collision_tiles, MapPlugin};
use player_car::{spawn_player_car, PlayerCarPlugin};
use resources::*;
use panel::PanelPlugin;
use semaphore::SemaphorePlugin;

use crate::chunk_manager::{setup_chunk_manager, ChunkManagerPlugin};

use super::GameGlobalState;

mod assets;
mod components;
mod constants;
mod entities;
mod resources;

pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayingData>()
            .insert_resource(Gravity::ZERO)
            .add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(PlayerCarPlugin)
            .add_plugins(PlayerCarExplosionPlugin)
            .add_plugins(MapPlugin)
            .add_plugins(ChunkManagerPlugin)
            .add_plugins(SemaphorePlugin)
            .add_plugins(PanelPlugin)
            .add_systems(OnEnter(GameGlobalState::Playing), on_enter)
            .add_systems(Update, (print_started_collisions,).run_if(in_state(GameGlobalState::Playing)));
    }
}

fn on_enter(
    mut playing_data: ResMut<PlayingData>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    *playing_data = Default::default();

    let map_data = playing_data.level.map_data();
    let initial_car_x = constants::MAP_ORIGIN_X + map_data.width / 2.0;

    let (config, initial_scroll) = build_map_config(&playing_data);
    setup_chunk_manager(config, initial_scroll, &mut commands);
    spawn_collision_tiles(&playing_data, &mut commands);
    spawn_player_car(&mut commands, &asset_server, &mut texture_atlas_layouts, initial_scroll, initial_car_x);
}

pub fn print_started_collisions(mut collision_event_reader: MessageReader<CollisionStart>) {
    for _ in collision_event_reader.read() {
        println!("Collision started");
    }
}
