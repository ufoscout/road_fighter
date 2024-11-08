use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::{components::*, constants::{PLAYER_BRAKE_RATE, PLAYER_MAX_ACCEL_RATE, PLAYER_MAX_HSPEED, PLAYER_MAX_SPEED, PLAYER_MIN_SPEED}, resources::*};

/// Move to menu screen whatever key is pressed
pub fn handle_key_pressed(
    time: Res<Time>,
    mut car: Query<(&mut PlayerOneCar)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let delta = time.delta_seconds();

    // for (mut car, mut velocity) in car.iter_mut() {

    let (mut car) = car.single_mut();
    let y_speed_ratio= (car.speed_y/PLAYER_MAX_SPEED).abs();

    let x_speed_ratio = if (y_speed_ratio<0.1) {
        y_speed_ratio*2.
    } else {
        (((y_speed_ratio-0.1)/0.9)*0.8)+0.2
    };

    if keyboard_input.pressed(KeyCode::Space) {
            car.speed_y += (1. - y_speed_ratio) * PLAYER_MAX_ACCEL_RATE * delta;
            car.speed_y = car.speed_y.min(PLAYER_MAX_SPEED);
            // println!("Increase Speed: {}", car.speed_y);
    } else {
            // println!("Brake: {}", car.speed_y);
            if car.speed_y < 0. {
                car.speed_y += PLAYER_BRAKE_RATE * delta;
            } else {
                car.speed_y -= PLAYER_BRAKE_RATE * delta;
            };

            // if speed is between -PLAYER_BRAKE_RATE and PLAYER_BRAKE_RATE, set it to 0
            if car.speed_y.abs() <= PLAYER_BRAKE_RATE {
                car.speed_y = 0.;
            }
    }
    
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
            car.speed_x = -x_speed_ratio * PLAYER_MAX_HSPEED;
    } else if keyboard_input.pressed(KeyCode::ArrowRight){
            car.speed_x = x_speed_ratio * PLAYER_MAX_HSPEED;
    } else {
            car.speed_x = 0.;
    }

    let position_ratio = 1. / 8.;
    car.y_position += car.speed_y * delta * position_ratio;
    car.x_position += car.speed_x * delta * position_ratio;
    // println!("Speed x: {}", car.speed_x);
    // println!("Posit y: {}", car.x_position);
    // }
}

pub fn wall_collisions(
    mut commands: Commands,
    map: Query<(&PlayingMap, &CollidingEntities)>,
) {
    for (map, collisions) in map.iter() {
        collisions.iter().for_each(|entity| {
            // println!("Map collision with entity {:?}", entity);
            commands.entity(*entity).insert(CollidedWithWall);
        });
    }
}

pub fn car_collided_with_wall(
    time: Res<Time>,
    mut car: Query<(&mut PlayerOneCar, &CollidedWithWall)>) {
        car.iter_mut().for_each(|(mut car, _)| {
            println!("Car collided with wall! BOOOOOM!!!!!!!");
            car.speed_x = 0.;
            car.speed_y = 0.;
        });
    }

pub fn render_screen(
    mut car: Query<(&mut PlayerOneCar, &mut Transform)>,
    mut map_data: Query<(&PlayingMap, &mut Transform), Without<PlayerOneCar>>,
) {

    let mut car = car.single_mut();
    car.1.translation.x = car.0.x_position; 

    for (map, mut transform) in map_data.iter_mut() {
        transform.translation.y = map.y_position - car.0.y_position;
    }
    
}

pub fn print_started_collisions(mut collision_event_reader: EventReader<CollisionStarted>) {
    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} started colliding",
            entity1,
            entity2,
        );
    }
}