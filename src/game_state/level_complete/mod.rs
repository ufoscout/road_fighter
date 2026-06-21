use bevy::prelude::*;

use crate::game_state::{
    playing::{PlayingData, PlayingLevel},
    GameGlobalState,
};

pub struct LevelCompletePlugin;

impl Plugin for LevelCompletePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameGlobalState::LevelComplete), on_enter);
    }
}

fn on_enter(
    mut playing_data: ResMut<PlayingData>,
    mut next_state: ResMut<NextState<GameGlobalState>>,
) {
    match playing_data.level.next() {
        Some(next_level) => {
            info!("Starting level {:?}", next_level);
            playing_data.level = next_level;
            next_state.set(GameGlobalState::Playing);
        }
        None => {
            info!("All levels complete! Returning to menu.");
            playing_data.level = PlayingLevel::LevelOne;
            next_state.set(GameGlobalState::Menu);
        }
    }
}
