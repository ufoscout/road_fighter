
use bevy::prelude::*;

use crate::game_state::{Game, GameState};

use super::components::DisclaimerBackground;

/// Hide the disclaimer screen when whatever key is pressed
pub fn disclaimer_key_pressed(
    mut game: ResMut<Game>, mut disclaimer_background: Query<(&mut DisclaimerBackground, &mut Visibility)>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.get_pressed().next().is_some() {
        game.state = GameState::Menu;
        disclaimer_background.iter_mut().for_each(|(_, mut visibility)| {
            *visibility = Visibility::Hidden;
        }); 
    }
}