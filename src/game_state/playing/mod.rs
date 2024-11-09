use avian2d::prelude::*;
use bevy::prelude::*;
use components::*;
use entities::*;
use explosion::PlayerCarExplosionPlugin;
use map::{span_map, MapPlugin};
use player_car::{spawn_player_car, PlayerCarPlugin};
use resources::*;

use super::GameGlobalState;

mod assets;
mod components;
mod constants;
mod entities;
mod resources;

/// The plugin that handles the Playing state
pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayingData>()
            .insert_resource(Gravity::ZERO)
            .add_plugins(PhysicsPlugins::default())
            // .add_plugins(PhysicsDebugPlugin::default())
            .add_plugins(PlayerCarPlugin)
            .add_plugins(PlayerCarExplosionPlugin)
            .add_plugins(MapPlugin)
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

    // Spawn the map
    let min_y = span_map(&playing_data, &mut commands, &asset_server, &mut texture_atlas_layouts);

    // Spawn the player car
    spawn_player_car(&mut commands, &asset_server, &mut texture_atlas_layouts, min_y);
}

pub fn print_started_collisions(mut collision_event_reader: EventReader<CollisionStarted>) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        println!("Entities {:?} and {:?} started colliding", entity1, entity2,);
    }
}
