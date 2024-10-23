use bevy::prelude::*;

pub mod map;
pub mod player_car;

/// All components for the Playing state
#[derive(Component)]
pub struct PlayingAll;

#[derive(Component)]
pub struct PlayerOneCar;
