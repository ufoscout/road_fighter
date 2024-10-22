use bevy::prelude::*;

// The global resource that holds the game state
#[derive(Resource, Default)]
pub struct IntroductionState {
    pub step: u8,
}
