// Player: 
// pub const PLAYER_MIN_SPEED: f32 = 0.;
pub const PLAYER_MAX_SPEED: f32 = 6144.;
pub const PLAYER_MAX_ACCEL_RATE: f32 = 48. * ORIGINAL_FPS;
pub const PLAYER_BRAKE_RATE: f32 = 16. * ORIGINAL_FPS;
// pub const PLAYER_BRAKE_RATE_NO_FUEL: f32 = 64. * ORIGINAL_FPS;
pub const PLAYER_MAX_HSPEED: f32 = 768.;
// pub const PLAYER_BOUNCE_HSPEED: usize = 880;
// pub const PLAYER_MAX_FUEL: usize = 2048;
// pub const PLAYER_FUEL_RECHARGE: usize = 384;
// pub const PLAYER_FUEL_LOSS: usize = 256;

// Enemy: 
// pub const ENEMY_SPEED: usize = 3328;
// pub const ENEMY_HSPEED: usize = 416;
// pub const ENEMY_CAR_INTERVAL: usize = 38;

// The original game runs at 27 FPS, so we need to adjust the speed of the game to match the original
pub const ORIGINAL_FPS: f32 = 27.;