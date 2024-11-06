use avian2d::prelude::*;
use bevy::prelude::*;

use super::*;


pub struct PlayerCar;

impl PlayerCar {

    pub fn spawn(&self, commands: &mut Commands, asset_server: &AssetServer, texture_atlas_layouts: &mut Assets<TextureAtlasLayout>, y_position: f32,) {

        // Spawn the player car
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("graphics/car1.png"),
                transform: Transform::from_translation(Vec3::new(0.0, -126., 255.0)),
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
            PlayerOneCar {
                y_position,
                x_position: 0.,
                speed_y: 0.,
                speed_x: 0.,
            },
            Collider::circle(10.),
            RigidBody::Kinematic,
            DebugRender::default().with_collider_color(Color::WHITE),
        ));

    }

    pub fn draw(&self) {
        // Draw the player car
    }

    pub fn update(&self) {
        
    }
}