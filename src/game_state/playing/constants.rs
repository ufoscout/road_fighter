// Player:
// pub const PLAYER_MIN_SPEED: f32 = 0.;
pub const PLAYER_MAX_SPEED: f32 = 6144.;
pub const PLAYER_MAX_ACCEL_RATE: f32 = 48. * ORIGINAL_FPS;
pub const PLAYER_BRAKE_RATE: f32 = 16. * ORIGINAL_FPS;
pub const PLAYER_BRAKE_RATE_NO_FUEL: f32 = 64. * ORIGINAL_FPS;
pub const PLAYER_MAX_HSPEED: f32 = 768.;
// pub const PLAYER_BOUNCE_HSPEED: f32 = 880.;
pub const PLAYER_MAX_FUEL: f32 = 2048.;
pub const PLAYER_FUEL_DRAIN_RATE: f32 = ORIGINAL_FPS;
pub const PLAYER_FUEL_RECHARGE: f32 = 384.;
pub const PLAYER_FUEL_LOSS_ON_CRASH: f32 = 256.;
pub const PLAYER_RESPAWN_DELAY_SECS: f32 = 2.;

// Enemy:
// pub const ENEMY_SPEED: usize = 3328;
// pub const ENEMY_HSPEED: usize = 416;
// pub const ENEMY_CAR_INTERVAL: usize = 38;

// Layout: the map's left edge aligns with the right edge of the left scoreboard panel.
// Left panel: 20 px wide, center at world x = -250  →  right edge = -250 + 10 = -240.
pub const MAP_ORIGIN_X: f32 = -240.;

// The original game has REDRAWING_PERIOD=27ms, giving ~37.037 FPS (27 is ms/frame, not FPS)
const ORIGINAL_FPS: f32 = 27.; // 1000. / 27.;
// Converts 8.8 fixed-point speed units (as used in the C original) to pixels/second
pub const PLAYER_POSITION_RATIO: f32 = ORIGINAL_FPS / 256.;
