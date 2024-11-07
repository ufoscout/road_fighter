use avian2d::prelude::Collider;
use bevy::{math::vec2, utils::hashbrown::HashMap};

use super::map::TileData;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct AssetKey<'a> {
    pub tile_source: &'a str,
    pub x: u32,
    pub y: u32,
}

pub fn colliders<'a>() -> HashMap<AssetKey<'a>, Collider> {
    let mut assets = HashMap::new();
    
    // Road left border
    {
        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 0,
            y: 128,
        }, Collider::polyline(vec![vec2(-5., -64.), vec2(-5., 64.) ], None));
        
        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 0,
            y: 256,
        }, Collider::polyline(vec![vec2(-5., -64.), vec2(-5., 64.) ], None));
        
        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 192,
            y: 128,
        }, Collider::polyline(vec![vec2(11., -64.), vec2(-5., 64.) ], None));
    
        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 192,
            y: 256,
        }, Collider::polyline(vec![vec2(-5., -64.), vec2(11., 64.) ], None));
            
    }
    
    // Road right border
    {

        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 96,
            y: 128,
        }, Collider::polyline(vec![vec2(4., -64.), vec2(4., 64.) ], None));
        
        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 96,
            y: 256,
        }, Collider::polyline(vec![vec2(4., -64.), vec2(4., 64.) ], None));

        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 288,
            y: 128,
        }, Collider::polyline(vec![vec2(4., -64.), vec2(-12., 64.) ], None));
    
        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 288,
            y: 256,
        }, Collider::polyline(vec![vec2(-12., -64.), vec2(4., 64.) ], None));
    }

    // Bridge
    {

        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 400,
            y: 128,
        }, Collider::polyline(vec![vec2(-11., -64.), vec2(-11., 64.) ], None));
        
        assets.insert(AssetKey {
            tile_source: "graphics/road.png",
            x: 400,
            y: 256,
        }, Collider::polyline(vec![vec2(-1., -64.), vec2(-1., 64.) ], None));

    }

    assets
}