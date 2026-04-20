use avian2d::prelude::PhysicsLayer;

pub mod explosion;
pub mod map;
pub mod player_car;

#[derive(PhysicsLayer, Default, Clone)]
pub enum GameLayer {
    #[default]
    Player, // Layer 0
    Wall,   // Layer 1
}
