use bevy::prelude::*;

use super::*;


pub struct PlayerCar {
}

impl PlayerCar {

    pub fn spawn(&self, commands: &mut Commands, asset_server: &AssetServer, texture_atlas_layouts: &mut Assets<TextureAtlasLayout>) {

        // Spawn the player car
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("graphics/car1.png"),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(32, 32),
                    1,
                    9,
                    None,
                    None,
                )),
            },
            PlayingAll,
            PlayerOneCar,
        ));

    }

    pub fn draw(&self) {
        // Draw the player car
    }

    pub fn update(&self) {
        
    }
}