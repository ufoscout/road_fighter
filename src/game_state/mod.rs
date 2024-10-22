use bevy::prelude::*;

pub mod disclaimer;
pub mod introduction;

// The global resource that holds the game state
#[derive(Resource, Default)]
pub struct Game {
    pub state: GameState,
}
 

// The possible states of the game
#[derive(Debug, PartialEq)]
pub enum GameState {
    // The initial disclaimer screen when the game starts
    Disclaimer,
    // The introduction screen
    Introduction,
    // The main menu screen
    Menu,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Disclaimer
    }
}