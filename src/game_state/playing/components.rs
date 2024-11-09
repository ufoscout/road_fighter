use bevy::prelude::*;

/// All components for the Playing state
#[derive(Component)]
pub struct PlayingAll;

#[derive(Component)]
pub struct PlayerOneCar {
    pub y_position: f32,
    pub x_position: f32,
    pub speed_x: f32,
    pub speed_y: f32,
}

#[derive(Component)]
pub struct PlayingMap {
    pub y_position: f32,
}

#[derive(Component)]
pub struct CollidedWithWall;

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
	fn default() -> Self {
		Self(Timer::from_seconds(0.1, TimerMode::Repeating))
	}
}