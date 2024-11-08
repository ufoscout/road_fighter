use std::{
    io::{BufRead, BufReader, Lines, Read},
    path::Path,
    str::{FromStr, SplitWhitespace},
    sync::OnceLock,
};

use bevy::prelude::*;

use crate::error::GameError;

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
            }
            PlayingLevel::LevelTwo => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level2.mg2").unwrap())
            }
            PlayingLevel::LevelThree => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level3.mg2").unwrap())
            }
            PlayingLevel::LevelFour => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level4.mg2").unwrap())
            }
            PlayingLevel::LevelFive => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level5.mg2").unwrap())
            }
            PlayingLevel::LevelSix => {
                static MAP: OnceLock<MapData> = OnceLock::new();
                MAP.get_or_init(|| MapData::from_file("data/maps/level6.mg2").unwrap())
            }
        }
    }
}

/// Represents the data of a map read from a mg2 file
#[derive(Debug, Default)]
pub struct MapData {
    pub tile_sources: Vec<String>,
    pub tiles: Vec<Vec<TileData>>,

    /// The index of the semaphore object in the objects field
    pub semaphore_object_index: Option<usize>,
    /// The semaphore tiles
    pub semaphore_tiles: [[usize; 2]; 5],

    pub background_tiles: Vec<MapTile>,
    pub middleground_tiles: Vec<MapTile>,
    pub foreground_tiles: Vec<MapTile>,

    pub width: f32,
    pub height: f32,
}

/// Represents the data of a tile read from a mg2 file
#[derive(Debug, Default)]
pub struct TileData {
    pub tile_source: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    // pub collision: bool,
}

/// Represents a tile in the map
#[derive(Debug, Default)]
pub struct MapTile {
    pub x: f32,
    pub y: f32,
    pub tile_bank: usize,
    pub tile_num: usize,
}

impl MapData {
    /// Load a map from a file
    pub fn from_file<P: AsRef<Path>>(file: P) -> Result<MapData, GameError> {
        let mut reader = std::fs::File::open(file)?;
        MapData::from_reader(&mut reader)
    }

    /// Load a map from a reader
    pub fn from_reader(reader: &mut dyn Read) -> Result<MapData, GameError> {
        let mut map = MapData::default();

        // Load tile sources
        let reader = BufReader::new(reader);
        let mut lines = reader.lines();

        // Read TILE_SOURCES block
        {
            let line = read_line(&mut lines)?;
            let mut line = line.split_whitespace();
            let _: String = parse_next(&mut line)?;
            let sources_count: usize = parse_next(&mut line)?;

            for _ in 0..sources_count {
                let line = read_line(&mut lines)?;
                let mut line = line.split_whitespace();
                let source: String = parse_next(&mut line)?;
                map.tile_sources.push(source);
            }
        }

        // Read TILES
        {
            // each map has exactly 256 tiles
            for _ in 0..256 {
                let mut tiles_bank = vec![];

                let line = read_line(&mut lines)?;
                let mut line: SplitWhitespace<'_> = line.split_whitespace();
                let _: String = parse_next(&mut line)?;
                let tiles_count: usize = parse_next(&mut line)?;

                for _ in 0..tiles_count {
                    // Firts line
                    let mut tile_data = TileData::default();
                    tile_data.tile_source = read_line(&mut lines)?;

                    // Second line
                    let line = read_line(&mut lines)?;
                    let mut line: SplitWhitespace<'_> = line.split_whitespace();

                    tile_data.x = parse_next(&mut line)?;
                    tile_data.y = parse_next(&mut line)?;
                    tile_data.width = parse_next(&mut line)?;
                    tile_data.height = parse_next(&mut line)?;

                    // Third line
                    let line = read_line(&mut lines)?;
                    let mut line: SplitWhitespace<'_> = line.split_whitespace();
                    let _ = parse_next::<u8>(&mut line);

                    // tile_data.collision = parse_next::<u8>(&mut line)? == 2;

                    // Add tile to map
                    tiles_bank.push(tile_data);
                }

                map.tiles.push(tiles_bank);
            }
        }

        // Read Objects block
        {
            let line = read_line(&mut lines)?;

            let mut line = line.split_whitespace();
            let _: String = parse_next(&mut line)?;
            let objects_count: usize = parse_next(&mut line)?;

            for object_index in 0..objects_count {
                // First line: object name
                let line = read_line(&mut lines)?;
                let mut line = line.split_whitespace();
                let _: String = parse_next(&mut line)?;
                let object_name: String = parse_next(&mut line)?;

                if object_name.to_lowercase() == "\"semaphore\"" {
                    map.semaphore_object_index = Some(object_index);
                }

                // Second line
                let line = read_line(&mut lines)?;
                let mut line = line.split_whitespace();
                let nbitmaps: usize = parse_next(&mut line)?;

                for nbitmap_count in 0..nbitmaps {
                    let line = read_line(&mut lines)?;
                    let mut line = line.split_whitespace();
                    let tile_bank: usize = parse_next(&mut line)?;
                    let tile_num: usize = parse_next(&mut line)?;

                    if Some(object_index) == map.semaphore_object_index {
                        map.semaphore_tiles[nbitmap_count][0] = tile_bank;
                        map.semaphore_tiles[nbitmap_count][1] = tile_num;
                    }

                    let nlinks: usize = parse_next(&mut line)?;

                    // skip lines
                    for _ in 0..nlinks {
                        let _ = read_line(&mut lines)?;
                    }
                }

                let line = read_line(&mut lines)?;
                let mut line = line.split_whitespace();
                let nparts: usize = parse_next(&mut line)?;

                // skip nparts lines
                for _ in 0..nparts {
                    let _ = read_line(&mut lines)?;
                }

                // skip 3 lines
                for _ in 0..3 {
                    let _ = read_line(&mut lines)?;
                }

                // skip 23 times nparts lines
                for _ in 0..23 {
                    for _ in 0..nparts {
                        let _ = read_line(&mut lines)?;
                    }
                }

                // after the first 'for' cicle, we are now at line 392 of the level1.map file

                // skip 8 lines
                for _ in 0..8 {
                    let _ = read_line(&mut lines)?;
                }
            }
        }

        // skip 7 lines
        for _ in 0..7 {
            let _ = read_line(&mut lines)?;
        }

        // The MAP section starts with a `SIZE number number` triplet in the map file
        {
            let line = read_line(&mut lines)?;
            let mut line = line.split_whitespace();
            let _: String = parse_next(&mut line)?;
            map.width = parse_next::<f32>(&mut line)? * 16.;
            map.height = parse_next::<f32>(&mut line)? * 16.;

            // skip 1 line
            let _ = read_line(&mut lines)?;

            // we are now at line 409 of the level1.map file
            // background tiles
            parse_objects_and_tiles(&mut lines, &mut map.background_tiles, map.semaphore_object_index)?;

            // we are now at line 1953 of the level1.map file
            // middleground
            parse_objects_and_tiles(&mut lines, &mut map.middleground_tiles, map.semaphore_object_index)?;

            // we are now at line 3925 of the level1.map file
            // foreground
            parse_objects_and_tiles(&mut lines, &mut map.foreground_tiles, map.semaphore_object_index)?;
        }

        Ok(map)
    }
}

/// Read objects and tiles from the map file
fn parse_objects_and_tiles(
    lines: &mut Lines<BufReader<&mut dyn Read>>,
    tiles: &mut Vec<MapTile>,
    semaphore_object_index: Option<usize>,
) -> Result<(), GameError> {
    // background tiles
    {
        let line = read_line(lines)?;
        // println!("background tiles line: {}", line);
        let mut line = line.split_whitespace();
        let count: usize = parse_next(&mut line)?;

        for _ in 0..count {
            let line = read_line(lines)?;
            let mut line = line.split_whitespace();

            let tile = MapTile {
                x: parse_next(&mut line)?,
                y: parse_next(&mut line)?,
                tile_bank: parse_next(&mut line)?,
                tile_num: parse_next(&mut line)?,
            };
            tiles.push(tile);
        }
    }

    // we are now at line 1951 of the level1.map file

    // background objects
    {
        let line = read_line(lines)?;
        // println!("background objects line: {}", line);
        let mut line = line.split_whitespace();
        let count: usize = parse_next(&mut line)?;

        for _ in 0..count {
            let line = read_line(lines)?;
            let mut line = line.split_whitespace();

            let _x: usize = parse_next(&mut line)?;
            let _y: usize = parse_next(&mut line)?;

            let line = read_line(lines)?;
            let mut line = line.split_whitespace();

            let index: usize = parse_next(&mut line)?;

            if Some(index) == semaphore_object_index {
                let TODO = 0;
                println!("TODO: semaphore background object found");
            }
        }
    }
    Ok(())
}

/// reads a line from the reader
fn read_line(reader: &mut Lines<BufReader<&mut dyn Read>>) -> Result<String, GameError> {
    let line = reader.next().ok_or_else(|| GameError::ParseError("Unexpected EOF".to_owned()))??;
    Ok(line)
}

/// reads a line from the reader and splits it into words
fn parse_next<T: FromStr>(split: &mut SplitWhitespace) -> Result<T, GameError>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let sources_count =
        split.next().ok_or_else(|| GameError::ParseError(format!("Expected no more split entries to parse")))?;
    let parsed: T =
        sources_count.parse().map_err(|err| GameError::ParseError(format!("Cannot parse as usize: {err:?}")))?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_map_from_file() {
        // Arrange
        let file: PathBuf = "data/maps/level1.mg2".into();
        assert!(file.exists());

        // Act
        let map = MapData::from_file(&file).unwrap();

        // Assert
        assert_eq!(map.tile_sources.len(), 3);
        assert_eq!(map.tile_sources[0], "graphics/road.png");

        assert_eq!(map.tiles.len(), 256);
        assert_eq!(map.tiles[0].len(), 11);

        // graphics/road.png
        // 0 256 32 128
        // 2 2
        assert_eq!(map.tiles[0][9].tile_source, "graphics/road.png");
        assert_eq!(map.tiles[0][9].x, 0);
        assert_eq!(map.tiles[0][9].y, 256);
        assert_eq!(map.tiles[0][9].width, 32);
        assert_eq!(map.tiles[0][9].height, 128);
        // assert_eq!(map.tiles[0][9].collision, true);

        // graphics/level1.png
        // 0 128 128 32
        // 2 0
        assert_eq!(map.tiles[1][1].tile_source, "graphics/level1.png");
        assert_eq!(map.tiles[1][1].x, 0);
        assert_eq!(map.tiles[1][1].y, 128);
        assert_eq!(map.tiles[1][1].width, 128);
        assert_eq!(map.tiles[1][1].height, 32);
        // assert_eq!(map.tiles[1][1].collision, false);

        assert_eq!(map.semaphore_object_index, Some(0));
        assert_eq!(map.semaphore_tiles[0], [1, 14]);
        assert_eq!(map.semaphore_tiles[1], [1, 15]);
        assert_eq!(map.semaphore_tiles[2], [1, 16]);
        assert_eq!(map.semaphore_tiles[3], [1, 17]);
        assert_eq!(map.semaphore_tiles[4], [1, 18]);

        assert_eq!(map.background_tiles.len(), 1541);

        // 128 16000 1 0
        assert_eq!(map.background_tiles[5].x, 128.);
        assert_eq!(map.background_tiles[5].y, 16000.);
        assert_eq!(map.background_tiles[5].tile_bank, 1);
        assert_eq!(map.background_tiles[5].tile_num, 0);

        assert_eq!(map.middleground_tiles.len(), 1970);

        // 160 16256 0 0
        assert_eq!(map.middleground_tiles[0].x, 160.);
        assert_eq!(map.middleground_tiles[0].y, 16256.);
        assert_eq!(map.middleground_tiles[0].tile_bank, 0);
        assert_eq!(map.middleground_tiles[0].tile_num, 0);

        assert_eq!(map.foreground_tiles.len(), 492);

        // 32 16256 1 6
        assert_eq!(map.foreground_tiles[1].x, 32.);
        assert_eq!(map.foreground_tiles[1].y, 16256.);
        assert_eq!(map.foreground_tiles[1].tile_bank, 1);
        assert_eq!(map.foreground_tiles[1].tile_num, 6);
    }

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
