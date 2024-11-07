use std::sync::OnceLock;

use bevy::prelude::*;

use crate::game_state::playing::components::map::MapData;

// The global resource that holds the game state
#[derive(Resource, Default)]
pub struct PlayingData {
    pub level: PlayingLevel,
}

// The different game levels
#[derive(Debug, PartialEq, Default)]
pub enum PlayingLevel {
    LevelOne,
    LevelTwo,
    #[default]
    LevelThree,
    LevelFour,
    LevelFive,
    LevelSix,
}

impl PlayingLevel {

    pub const ALL: [PlayingLevel; 6] = [
        PlayingLevel::LevelOne,
        PlayingLevel::LevelTwo,
        PlayingLevel::LevelThree,
        PlayingLevel::LevelFour,
        PlayingLevel::LevelFive,
        PlayingLevel::LevelSix,
    ];

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
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_parse_all_maps() {
        for level in PlayingLevel::ALL.iter() {
            level.map_data();
        }
    }

    #[test]
    fn test_all_assets_exists() {

        let check_assets = |level: &PlayingLevel| {
            let map_data = level.map_data();
            
            for asset in &map_data.tile_sources {
                let path: PathBuf = format!("assets/{asset}").into();
                assert!(path.exists());
            }
        };

        for level in PlayingLevel::ALL.iter() {
            check_assets(level);
        }
        
    }
}