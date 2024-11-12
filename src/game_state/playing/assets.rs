use avian2d::prelude::*;
use bevy::{ecs::system::EntityCommands, math::vec2};

use super::{GameLayer, LeftWall, RightWall};

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct AssetKey<'a> {
    pub tile_source: &'a str,
    pub x: u32,
    pub y: u32,
}

pub fn add_collider(commands: &mut EntityCommands, asset_key: AssetKey) {
    
    match asset_key {
        // Road left border
        AssetKey { tile_source: "graphics/road.png", x: 0, y: 128 } => {
            commands
                .insert(Collider::polyline(vec![vec2(-5., -64.), vec2(-5., 63.)], None))
                .insert(LeftWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },
        AssetKey { tile_source: "graphics/road.png", x: 0, y: 256 } => {
            commands
            .insert(Collider::polyline(vec![vec2(-5., -64.), vec2(-5., 63.)], None))
            .insert(LeftWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },
        AssetKey { tile_source: "graphics/road.png", x: 192, y: 128 } => {
            commands.insert(Collider::polyline(vec![vec2(11., -64.), vec2(-5., 63.)], None))
            .insert(LeftWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },
        AssetKey { tile_source: "graphics/road.png", x: 192, y: 256 } => {
            commands.insert(Collider::polyline(vec![vec2(-5., -64.), vec2(11., 63.)], None))
            .insert(LeftWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },

        // Road right border
        AssetKey { tile_source: "graphics/road.png", x: 96, y: 128 } => {
            commands.insert(Collider::polyline(vec![vec2(4., -64.), vec2(4., 63.)], None))
            .insert(RightWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },
        AssetKey { tile_source: "graphics/road.png", x: 96, y: 256 } => {
            commands.insert(Collider::polyline(vec![vec2(4., -64.), vec2(4., 63.)], None))
            .insert(RightWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },
        AssetKey { tile_source: "graphics/road.png", x: 288, y: 128 } => {
            commands.insert(Collider::polyline(vec![vec2(4., -64.), vec2(-12., 63.)], None))
            .insert(RightWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },
        AssetKey { tile_source: "graphics/road.png", x: 288, y: 256 } => {
            commands.insert(Collider::polyline(vec![vec2(-12., -64.), vec2(4., 63.)], None))
            .insert(RightWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },

        // Bridge
        AssetKey { tile_source: "graphics/road.png", x: 400, y: 128 } => {
            commands.insert(Collider::polyline(vec![vec2(-11., -64.), vec2(-11., 63.)], None))
            .insert(LeftWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },
        AssetKey { tile_source: "graphics/road.png", x: 400, y: 256 } => {
            commands.insert(Collider::polyline(vec![vec2(-1., -64.), vec2(-1., 63.)], None))
            .insert(RightWall)
                .insert(CollisionLayers::new([GameLayer::Wall], [GameLayer::Player]))
                .insert(RigidBody::Static);
        },

        _ => {}
    }
    
}
