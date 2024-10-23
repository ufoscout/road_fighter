use std::{io::{BufRead, BufReader, Lines, Read}, path::Path, str::{FromStr, SplitWhitespace}};

use crate::error::GameError;

#[derive(Debug, Default)]
pub struct MapData {
    pub tile_sources: Vec<String>,
    pub tiles: Vec<TileData>,

}

#[derive(Debug, Default)]
pub struct TileData {
    pub tile_source: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub collision: bool,
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
            for _ in 0..255 {

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

                    tile_data.collision = parse_next::<u8>(&mut line)? == 2;

                    // Add tile to map
                    map.tiles.push(tile_data);
                }
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

/// reads a line from the reader
fn read_line(reader: &mut Lines<BufReader<&mut dyn Read>>) -> Result<String, GameError> {
    let line = reader.next().ok_or_else(|| GameError::ParseError("Unexpected EOF".to_owned()))??;
    Ok(line)
}

/// reads a line from the reader and splits it into words
fn parse_next<T: FromStr>(split: &mut SplitWhitespace) -> Result<T, GameError> 
    where <T as FromStr>::Err: std::fmt::Debug
{
    let sources_count = split.next().ok_or_else(|| GameError::ParseError(format!("Expected no more split entries to parse")))?;
    let parsed: T = sources_count.parse().map_err(|err| GameError::ParseError(format!("Cannot parse as usize: {err:?}")))?;
    Ok(parsed)
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

        assert_eq!(map.tiles.len(), 30);
        
        // graphics/road.bmp
        // 0 256 32 128
        // 2 2
        assert_eq!(map.tiles[9].tile_source, "graphics/road.bmp");
        assert_eq!(map.tiles[9].x, 0);
        assert_eq!(map.tiles[9].y, 256);
        assert_eq!(map.tiles[9].width, 32);
        assert_eq!(map.tiles[9].height, 128);
        assert_eq!(map.tiles[9].collision, true);

        // graphics/level1.bmp
        // 0 128 128 32
        // 2 0
        assert_eq!(map.tiles[12].tile_source, "graphics/level1.bmp");
        assert_eq!(map.tiles[12].x, 0);
        assert_eq!(map.tiles[12].y, 128);
        assert_eq!(map.tiles[12].width, 128);
        assert_eq!(map.tiles[12].height, 32);
        assert_eq!(map.tiles[12].collision, false);

    }

}