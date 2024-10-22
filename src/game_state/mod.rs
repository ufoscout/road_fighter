use bevy::prelude::*;

pub mod disclaimer;
pub mod introduction;
pub mod menu;

// The possible states of the game
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameGlobalState {
    // The initial disclaimer screen when the game starts
    #[default]
    Disclaimer,
    // The introduction screen
    Introduction,
    // The main menu screen
    Menu,
}
