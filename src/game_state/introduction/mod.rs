use bevy::prelude::*;
use components::*;
use state::IntroductionState;

use super::GameGlobalState;

mod components;
mod state;
mod systems;

/// The plugin that sets up the disclaimer screen
pub struct IntroductionPlugin;

impl Plugin for IntroductionPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<IntroductionState>()
        .add_systems(OnEnter(GameGlobalState::Introduction), setup)
        .add_systems(Update, systems::introduction_key_pressed.run_if(in_state(GameGlobalState::Introduction)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // Spawn the background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/retroremakes.png"),
            ..default()
        },
        IntroductionBackground
    ));

}