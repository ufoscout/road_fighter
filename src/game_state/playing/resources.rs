use std::{cell::OnceCell, sync::OnceLock};

use bevy::{prelude::*, scene::ron::Map, tasks::futures_lite::stream::Once};

use crate::game_state::playing::components::map::MapData;

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

    /// Returns the map data for the level
    pub fn map_data(&self) -> &MapData {
        match self {
            PlayingLevel::LevelOne => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level1.mg2").unwrap())
            },
            PlayingLevel::LevelTwo => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level2.mg2").unwrap())
            },
            PlayingLevel::LevelThree => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level3.mg2").unwrap())
            },
            PlayingLevel::LevelFour => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level4.mg2").unwrap())
            },
            PlayingLevel::LevelFive => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level5.mg2").unwrap())
            },
            PlayingLevel::LevelSix => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level6.mg2").unwrap())
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_maps() {
        PlayingLevel::LevelOne.map_data();
        PlayingLevel::LevelTwo.map_data();
        PlayingLevel::LevelThree.map_data();
        PlayingLevel::LevelFour.map_data();
        PlayingLevel::LevelFive.map_data();
        PlayingLevel::LevelSix.map_data();
    }
}