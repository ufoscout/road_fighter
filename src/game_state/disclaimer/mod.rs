use bevy::prelude::*;
use components::{DisclaimerAll, DisclaimerBackground};

use super::GameGlobalState;

mod components;
mod systems;

/// The plugin that sets up the disclaimer screen
pub struct DisclaimerStatePlugin;

impl Plugin for DisclaimerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameGlobalState::Disclaimer), on_enter)
            .add_systems(OnExit(GameGlobalState::Disclaimer), on_exit)
            .add_systems(Update, systems::disclaimer_key_pressed.run_if(in_state(GameGlobalState::Disclaimer)));
    }
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the background
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/disclaimer.png"),
            ..default()
        },
        DisclaimerAll,
        DisclaimerBackground,
    ));
}

// Despawn the disclaimer screen
fn on_exit(mut commands: Commands, disclaimer_all: Query<(Entity, &DisclaimerAll)>) {
    disclaimer_all.iter().for_each(|(entity, _)| commands.entity(entity).despawn());
}
