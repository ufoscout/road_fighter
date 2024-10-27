use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::{components::*, resources::*};

/// Move to menu screen whatever key is pressed
pub fn handle_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    mut playing_data: ResMut<PlayingData>,
    mut car: Query<&mut PlayerOneCar>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {

    for mut car in car.iter_mut() {
        car.speed_y = 0.;
    }

    if keyboard_input.pressed(KeyCode::ArrowUp) {
        for mut car in car.iter_mut() {
            car.speed_y = -15.;
        }
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        for mut car in car.iter_mut() {
            car.speed_y = 15.;
        }
    } else if keyboard_input.pressed(KeyCode::ArrowLeft) {

    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        
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

    let race_y_position = car.single().1.speed_y;

    for (_, mut transform) in map_data.iter_mut() {
        transform.translation.y += race_y_position;
    }
    
}