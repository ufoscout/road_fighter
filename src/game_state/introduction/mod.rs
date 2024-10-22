use bevy::prelude::*;
use components::*;
use state::IntroductionState;

use super::GameGlobalState;

mod components;
mod state;
mod systems;

/// The plugin that sets up the disclaimer screen
pub struct IntroductionStatePlugin;

impl Plugin for IntroductionStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IntroductionState>()
            .add_systems(OnEnter(GameGlobalState::Introduction), on_enter)
            .add_systems(OnExit(GameGlobalState::Introduction), on_exit)
            .add_systems(
                Update,
                systems::introduction_key_pressed.run_if(in_state(GameGlobalState::Introduction)),
            );
    }
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/retroremakes.png"),
            ..default()
        },
        IntroductionBackground,
    ));
}

fn on_exit(mut intro_state: ResMut<IntroductionState>) {
    *intro_state = Default::default();
}