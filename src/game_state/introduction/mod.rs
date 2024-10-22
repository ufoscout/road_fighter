use bevy::prelude::*;
use components::*;
use state::IntroductionState;

use super::{Game, GameState};

mod components;
mod state;
mod systems;

/// The plugin that sets up the disclaimer screen
pub struct IntroductionPlugin;

impl Plugin for IntroductionPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<IntroductionState>()
        .add_systems(Startup, setup)
        .add_systems(Update, systems::introduction_key_pressed.run_if(is_state_introduction));
    }
}

fn is_state_introduction(game: Res<Game>) -> bool {
    game.state == GameState::Introduction
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    // texture_atlas_builder.add_texture(Some(image1_id), image1);

    // // Spawn the background2
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("graphics/retroremakes.png"),
    //         sprite: Sprite {
    //             ..default() // Everything else is set to default
    //         },
    //         visibility: Visibility::Hidden,
    //         ..default()
    //     },
    //     IntroductionBackground
    // ));

    // // Spawn the background2
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("graphics/konami2.png"),
    //         sprite: Sprite {
    //             ..default() // Everything else is set to default
    //         },
    //         visibility: Visibility::Hidden,
    //         ..default()
    //     },
    //     IntroductionBackground
    // ));
}