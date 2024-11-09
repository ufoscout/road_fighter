use avian2d::prelude::PhysicsLayer;

pub mod explosion;
pub mod map;
pub mod player_car;

#[derive(PhysicsLayer)]
pub enum GameLayer {
    Player, // Layer 0
    Wall,   // Layer 1
}
