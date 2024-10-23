use std::{io::{BufRead, BufReader, Read}, path::Path};

use crate::error::GameError;

#[derive(Debug, Default)]
pub struct MapData {
    pub tile_sources: Vec<String>,

}

impl MapData {

    pub fn from_file(file: &Path) -> Result<MapData, GameError> {
        let mut reader = std::fs::File::open(file)?;
        MapData::from_reader(&mut reader)
    }

    pub fn from_reader(reader: &mut dyn Read) -> Result<MapData, GameError> {

        let mut map = MapData::default();

        // Load tile sources
        let reader = BufReader::new(reader);
        let mut lines = reader.lines();

        // Read TILE_SOURCES block
        {
            let line = lines.next().ok_or_else(|| GameError::ParseError("Expected TILE_SOURCES block".to_owned()))??;
            let sources_count = line.split_whitespace().nth(1).ok_or_else(|| GameError::ParseError("Expected TILE_SOURCES block".to_owned()))?;
            let sources_count: usize = sources_count.parse().map_err(|err| GameError::ParseError(format!("Cannot parse as usize: {err:?}")))?;

            for _ in 0..sources_count {
                let line = lines.next().ok_or_else(|| GameError::ParseError(format!("Expected {sources_count} entries in TILE_SOURCES block")))??;
                let source = line.split_whitespace().nth(0).ok_or_else(|| GameError::ParseError("Expected tile name in TILE_SOURCES block".to_owned()))?;
                map.tile_sources.push(source.to_owned());
            }
        }
        // let line = lines.next()?;
        
        // let n: usize = line.split_whitespace().nth(1)?.parse().ok()?;
        // for _ in 0..n {
        //     let source = TileSource;
        //     source.load(&mut reader);
        //     self.tile_sources.push(source);
        // }

        Ok(map)
    }

}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::game_state::playing::PlayingLevel;

    use super::*;

    #[test]
    fn test_map_from_file() {
        // Arrange
        let file: PathBuf = PlayingLevel::LevelOne.map_file().into();
        assert!(file.exists());

        // Act
        let map = MapData::from_file(&file).unwrap();

        // Assert
        assert_eq!(map.tile_sources.len(), 3);
        assert_eq!(map.tile_sources[0], "graphics/road.bmp");
    }

}