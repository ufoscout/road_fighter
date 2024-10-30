use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::{components::*, constants::{PLAYER_BRAKE_RATE, PLAYER_MAX_ACCEL_RATE, PLAYER_MAX_SPEED, PLAYER_MIN_SPEED}, resources::*};

/// Move to menu screen whatever key is pressed
pub fn handle_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    mut playing_data: ResMut<PlayingData>,
    mut car: Query<&mut PlayerOneCar>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {

    
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for mut car in car.iter_mut() {
            let speed_ration= car.speed_y/PLAYER_MAX_SPEED;
            car.speed_y += (1. - speed_ration.abs()) * PLAYER_MAX_ACCEL_RATE;
            car.speed_y = car.speed_y.min(PLAYER_MAX_SPEED);
            println!("Increase Speed: {}", car.speed_y);
        }
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        for mut car in car.iter_mut() {
            let speed_ration= car.speed_y/PLAYER_MAX_SPEED;
            car.speed_y -= (1. - speed_ration.abs()) * PLAYER_MAX_ACCEL_RATE;
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

    } 
    
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        
    }

}

/// Move to menu screen whatever key is pressed
pub fn render_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    introduction_state: Res<PlayingData>,
    car: Query<(Entity, &PlayerOneCar)>,
    mut map_data: Query<(&PlayingMap, &mut Transform)>,
) {

    let race_y_position = car.single().1.speed_y / 100.;

    for (_, mut transform) in map_data.iter_mut() {
        transform.translation.y -= race_y_position;
    }
    
}