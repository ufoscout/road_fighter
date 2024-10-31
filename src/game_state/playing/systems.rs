use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::{components::*, constants::{PLAYER_BRAKE_RATE, PLAYER_MAX_ACCEL_RATE, PLAYER_MAX_HSPEED, PLAYER_MAX_SPEED, PLAYER_MIN_SPEED}, resources::*};

/// Move to menu screen whatever key is pressed
pub fn handle_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    mut playing_data: ResMut<PlayingData>,
    mut car: Query<&mut PlayerOneCar>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {

    
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for mut car in car.iter_mut() {
            let speed_ratio= car.speed_y/PLAYER_MAX_SPEED;
            car.speed_y += (1. - speed_ratio.abs()) * PLAYER_MAX_ACCEL_RATE;
            car.speed_y = car.speed_y.min(PLAYER_MAX_SPEED);
            println!("Increase Speed: {}", car.speed_y);
        }
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        for mut car in car.iter_mut() {
            let speed_ratio= car.speed_y/PLAYER_MAX_SPEED;
            car.speed_y -= (1. - speed_ratio.abs()) * PLAYER_MAX_ACCEL_RATE;
            car.speed_y = car.speed_y.max(-PLAYER_MAX_SPEED);
            println!("Decrease Speed: {}", car.speed_y);
        }
    } else {
        for mut car in car.iter_mut() {
            println!("Brake: {}", car.speed_y);
            if car.speed_y < 0. {
                car.speed_y += PLAYER_BRAKE_RATE;
            } else {
                car.speed_y -= PLAYER_BRAKE_RATE;
            };

            // if speed is between -PLAYER_BRAKE_RATE and PLAYER_BRAKE_RATE, set it to 0
            if car.speed_y.abs() <= PLAYER_BRAKE_RATE {
                car.speed_y = 0.;
            }

        }
    }

    
    
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        for mut car in car.iter_mut() {
            let speed_ratio= (car.speed_y/PLAYER_MAX_SPEED).abs();
            let speed_ratio = if (speed_ratio<0.1) {
                speed_ratio*2.
            } else {
                (((speed_ratio-0.1)/0.9)*0.8)+0.2
            };
            car.speed_x -= speed_ratio * PLAYER_MAX_HSPEED;
            car.speed_x = car.speed_x.max(-PLAYER_MAX_HSPEED);
        }
    } else if keyboard_input.pressed(KeyCode::ArrowRight){
        for mut car in car.iter_mut() {
            let speed_ratio= (car.speed_y/PLAYER_MAX_SPEED).abs();
            let speed_ratio = if (speed_ratio<0.1) {
                speed_ratio*2.
            } else {
                (((speed_ratio-0.1)/0.9)*0.8)+0.2
            };
            car.speed_x += speed_ratio * PLAYER_MAX_HSPEED;
            car.speed_x = car.speed_x.min(PLAYER_MAX_HSPEED);
        }
    } else {
        for mut car in car.iter_mut() {
                car.speed_x = 0.;
        }
    }

}

/// Move to menu screen whatever key is pressed
pub fn render_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    introduction_state: Res<PlayingData>,
    mut car: Query<(&mut PlayerOneCar, &mut Transform)>,
    mut map_data: Query<(&PlayingMap, &mut Transform), Without<PlayerOneCar>>,
) {

    let mut car = car.single_mut();
    let race_y_position = car.0.speed_y / 500.;
    let race_x_position = car.0.speed_x / 500.;
    car.1.translation.x += race_x_position; 

    for (_, mut transform) in map_data.iter_mut() {
        transform.translation.y -= race_y_position;
    }
    
}