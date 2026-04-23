use avian2d::prelude::*;
use bevy::{math::vec2, prelude::*};
use explosion::spawn_explosion;

use crate::game_state::{
    playing::{
        constants::{
            PLAYER_BRAKE_RATE, PLAYER_BRAKE_RATE_NO_FUEL, PLAYER_FUEL_DRAIN_RATE,
            PLAYER_FUEL_LOSS_ON_CRASH, PLAYER_MAX_ACCEL_RATE, PLAYER_MAX_FUEL, PLAYER_MAX_HSPEED,
            PLAYER_MAX_SPEED, PLAYER_POSITION_RATIO, PLAYER_RESPAWN_DELAY_SECS,
        },
        CarCollidedSide, CollidedWithWall, PlayerOneCar, PlayingAll, ToBeRespawned,
    },
    GameGlobalState,
};
use crate::game_state::playing::resources::{PlayingData, RaceState};

use super::*;

/// The plugin that handles the player car
pub struct PlayerCarPlugin;

impl Plugin for PlayerCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_key_pressed, drain_fuel, render_screen, car_collided_with_wall, respawn_player_car).run_if(in_state(GameGlobalState::Playing)),
        );
    }
}

pub fn spawn_player_car(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    y_position: f32,
    x_position: f32,
) {
    // Spawn the player car
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/car1.png"),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 9, None, None)),
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -126., 255.0)),
        PlayingAll,
        PlayerOneCar { y_position, x_position, speed_y: 0., speed_x: 0., fuel: PLAYER_MAX_FUEL * 0.95 },
        Collider::polyline(
            vec![
                vec2(-9., -15.),
                vec2(-9., 10.),
                vec2(-5., 15.),
                vec2(2., 15.),
                vec2(6., 10.),
                vec2(6., -15.),
                vec2(-9., -15.),
            ],
            None,
        ),
        RigidBody::Kinematic,
        CollisionLayers::new(GameLayer::Player, [GameLayer::Wall]),
        // DebugRender::default().with_collider_color(Color::WHITE),
    ));
}

pub fn respawn_player_car(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    car: Query<(Entity, &ToBeRespawned)>) {

    let now = time.elapsed_secs();
    for (id, car) in car.iter() {
        if car.despawn_time + PLAYER_RESPAWN_DELAY_SECS < now {
            let car = car.car.clone();
            commands.entity(id).despawn();
            spawn_player_car(&mut commands, &asset_server, &mut texture_atlas_layouts, car.y_position, car.x_position);
        }
    }
}

pub fn handle_key_pressed(
    time: Res<Time>,
    mut car: Query<&mut PlayerOneCar>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    playing_data: Res<PlayingData>,
) {
    if playing_data.race_state != RaceState::Started {
        return;
    }

    let delta = time.delta_secs();

    for mut car in car.iter_mut() {
        let y_speed_ratio = (car.speed_y / PLAYER_MAX_SPEED).abs();

        let x_speed_ratio =
            if y_speed_ratio < 0.1 { y_speed_ratio * 2. } else { (((y_speed_ratio - 0.1) / 0.9) * 0.8) + 0.2 };

        if car.fuel > 0. {
            if keyboard_input.pressed(KeyCode::Space) {
                car.speed_y += (1. - y_speed_ratio) * PLAYER_MAX_ACCEL_RATE * delta;
                car.speed_y = car.speed_y.min(PLAYER_MAX_SPEED);
            } else {
                let brake = PLAYER_BRAKE_RATE * delta;
                if car.speed_y.abs() <= brake {
                    car.speed_y = 0.;
                } else if car.speed_y > 0. {
                    car.speed_y -= brake;
                } else {
                    car.speed_y += brake;
                }
            }
        } else {
            let brake = PLAYER_BRAKE_RATE_NO_FUEL * delta;
            if car.speed_y.abs() <= brake {
                car.speed_y = 0.;
            } else if car.speed_y > 0. {
                car.speed_y -= brake;
            } else {
                car.speed_y += brake;
            }
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            car.speed_x = -x_speed_ratio * PLAYER_MAX_HSPEED;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            car.speed_x = x_speed_ratio * PLAYER_MAX_HSPEED;
        } else {
            car.speed_x = 0.;
        }

        car.y_position += car.speed_y * delta * PLAYER_POSITION_RATIO;
        car.x_position += car.speed_x * delta * PLAYER_POSITION_RATIO;
        // println!("Speed x: {}", car.speed_x);
        // println!("Posit y: {}", car.x_position);
    }
}

pub fn car_collided_with_wall(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut car: Query<(Entity, &mut PlayerOneCar, &Transform, &CollidedWithWall)>) {
    car.iter_mut().for_each(|(id, mut car, transform, collision)| {

        car.speed_x = 0.;
        car.speed_y = 0.;

        if car.fuel > PLAYER_FUEL_LOSS_ON_CRASH * 2. {
            car.fuel -= PLAYER_FUEL_LOSS_ON_CRASH;
        }

        match collision.side {
            CarCollidedSide::Left => car.x_position += 20.,
            CarCollidedSide::Right => car.x_position -= 20.,
        }

        commands.entity(id).despawn();
        commands.spawn(ToBeRespawned { car: car.clone(), despawn_time: time.elapsed_secs() });
        spawn_explosion(&mut commands, &asset_server, &mut texture_atlas_layouts, transform.translation);
    });
}

pub fn render_screen(mut car: Query<(&mut PlayerOneCar, &mut Transform)>) {
    car.iter_mut().for_each(|(car, mut transform)| {
        transform.translation.x = car.x_position;
    });
}

pub fn drain_fuel(
    time: Res<Time>,
    playing_data: Res<PlayingData>,
    mut car: Query<&mut PlayerOneCar>,
) {
    if playing_data.race_state != RaceState::Started {
        return;
    }
    let delta = time.delta_secs();
    for mut car in car.iter_mut() {
        car.fuel = (car.fuel - PLAYER_FUEL_DRAIN_RATE * delta).max(0.);
    }
}
