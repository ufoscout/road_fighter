use bevy::prelude::*;
use components::DisclaimerBackground;

mod components;
mod systems;

/// The plugin that sets up the disclaimer screen
pub struct DisclaimerPlugin;

impl Plugin for DisclaimerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, systems::disclaimer_key_pressed);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("graphics/disclaimer.png"),
            sprite: Sprite {
                // custom_size: Some(Vec2::new(WINDOW_WIDTH + BACKGROUNG_IMAGE_WIDTH * 2., WINDOW_HEIGHT)), // Adding a custom size
                ..default() // Everything else is set to default
            },
            visibility: Visibility::Visible,
            ..default()
        },
        DisclaimerBackground
    ));
}