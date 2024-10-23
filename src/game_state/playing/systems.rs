use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::{components::*, resources::*};

/// Move to menu screen whatever key is pressed
pub fn handle_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    mut playing_data: ResMut<PlayingData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {

}

/// Move to menu screen whatever key is pressed
pub fn render_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    introduction_state: Res<PlayingData>,
    background: Query<(Entity, &PlayerOneCar)>,
) {

    
    
}