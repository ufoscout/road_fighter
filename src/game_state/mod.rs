use bevy::prelude::*;

pub mod disclaimer;
pub mod introduction;
pub mod level_complete;
pub mod menu;
pub mod playing;

// The possible states of the game
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameGlobalState {
    // The initial disclaimer screen when the game starts
    Disclaimer,
    // The introduction screen
    Introduction,
    // The main menu screen
    Menu,
    // Play the game
    #[default]
    Playing,
    // Brief transition state between levels
    LevelComplete,
}
