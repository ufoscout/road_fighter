use bevy::prelude::*;

use crate::game_state::GameGlobalState;

/// Hide the disclaimer screen when whatever key is pressed
pub fn on_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
}
