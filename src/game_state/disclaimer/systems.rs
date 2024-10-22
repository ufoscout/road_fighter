use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::components::DisclaimerBackground;

/// Hide the disclaimer screen when whatever key is pressed
pub fn disclaimer_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    mut disclaimer_background: Query<(&mut DisclaimerBackground, &mut Visibility)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.get_pressed().next().is_some() {
        next_state.set(GameGlobalState::Introduction);
        disclaimer_background
            .iter_mut()
            .for_each(|(_, mut visibility)| {
                *visibility = Visibility::Hidden;
            });
    }
}
