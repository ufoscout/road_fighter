use bevy::prelude::*;
use components::player_car::PlayerCar;
use resources::*;

use super::GameGlobalState;

mod components;
mod resources;
mod systems;

/// The plugin that handles the Playing state
pub struct PlayingStatePlugin;

impl Plugin for PlayingStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayingData>()
            .add_systems(OnEnter(GameGlobalState::Playing), on_enter)
            .add_systems(
                Update,
                (systems::handle_key_pressed, systems::render_screen).run_if(in_state(GameGlobalState::Playing)),
            );
    }
}

fn on_enter(mut playing_state: ResMut<PlayingData>,
    mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,) {
    *playing_state = Default::default();

    // Preload the map assets
    

    // Spawn the player car
    PlayerCar{}.spawn(&mut commands, &asset_server, &mut texture_atlas_layouts);
}
