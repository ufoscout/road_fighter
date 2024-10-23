use bevy::prelude::*;

// The global resource that holds the game state
#[derive(Resource, Default)]
pub struct PlayingData {
    pub level: PlayingLevel,
}

// The different game levels
#[derive(Debug, PartialEq, Default)]
pub enum PlayingLevel {
    #[default]
    LevelOne,
    LevelTwo,
    LevelThree,
    LevelFour,
    LevelFive,
    LevelSix,
}

impl PlayingLevel {

    /// Returns the path to the map file for the level
    pub fn map_file(&self) -> &str {
        match self {
            PlayingLevel::LevelOne => "data/maps/level1.mg2",
            PlayingLevel::LevelTwo => "data/maps/level2.mg2",
            PlayingLevel::LevelThree => "data/maps/level3.mg2",
            PlayingLevel::LevelFour => "data/maps/level4.mg2",
            PlayingLevel::LevelFive => "data/maps/level5.mg2",
            PlayingLevel::LevelSix => "data/maps/level6.mg2",
        }
    }
}
