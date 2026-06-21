use avian2d::prelude::*;
use bevy::{math::vec2, prelude::*};
use explosion::spawn_explosion;

use crate::game_state::{
    playing::{
        constants::{
            PLAYER_BRAKE_RATE, PLAYER_BRAKE_RATE_NO_FUEL, PLAYER_FUEL_DRAIN_RATE,
            PLAYER_FUEL_LOSS_ON_CRASH, PLAYER_MAX_ACCEL_RATE, PLAYER_MAX_FUEL, PLAYER_MAX_HSPEED,
            PLAYER_MAX_SPEED, PLAYER_POSITION_RATIO, PLAYER_RESPAWN_DELAY_SECS,
        },
        CarCollidedSide, CollidedWithWall, PlayerOneCar, PlayingAll, ToBeRespawned,
    },
    GameGlobalState,
};
use crate::game_state::playing::resources::{PlayingData, RaceState};

use super::{map::wall_distances, *};

/// The plugin that handles the player car
pub struct PlayerCarPlugin;

impl Plugin for PlayerCarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_key_pressed, drain_fuel, render_screen, car_collided_with_wall, respawn_player_car, check_level_complete).run_if(in_state(GameGlobalState::Playing)),
        );
    }
}

pub fn spawn_player_car(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    y_position: f32,
    x_position: f32,
) {
    // Spawn the player car
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/car1.png"),
            texture_atlas: Some(TextureAtlas {
                index: 0,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(32, 32), 1, 9, None, None)),
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -126., 255.0)),
        PlayingAll,
        PlayerOneCar { y_position, x_position, speed_y: 0., speed_x: 0., fuel: PLAYER_MAX_FUEL * 0.95 },
        Collider::polyline(
            vec![
                vec2(-9., -15.),
                vec2(-9., 10.),
                vec2(-5., 15.),
                vec2(2., 15.),
                vec2(6., 10.),
                vec2(6., -15.),
                vec2(-9., -15.),
            ],
            None,
        ),
        RigidBody::Kinematic,
        CollisionLayers::new(GameLayer::Player, [GameLayer::Wall]),
        // DebugRender::default().with_collider_color(Color::WHITE),
    ));
}

pub fn respawn_player_car(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    spatial_query: SpatialQuery,
    car: Query<(Entity, &ToBeRespawned)>,
) {
    let now = time.elapsed_secs();
    for (id, car) in car.iter() {
        if car.despawn_time + PLAYER_RESPAWN_DELAY_SECS < now {
            let car = car.car.clone();
            let probe = Transform::from_translation(Vec3::new(car.x_position, -126.0, 0.0));
            let x_position = wall_distances(&probe, &spatial_query)
                .map(|(left_dist, right_dist)| car.x_position + (right_dist - left_dist) / 2.0)
                .unwrap_or(car.x_position);
            commands.entity(id).despawn();
            spawn_player_car(&mut commands, &asset_server, &mut texture_atlas_layouts, car.y_position, x_position);
        }
    }
}

pub fn handle_key_pressed(
    time: Res<Time>,
    mut car: Query<&mut PlayerOneCar>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    playing_data: Res<PlayingData>,
) {
    if playing_data.race_state != RaceState::Started {
        return;
    }

    let delta = time.delta_secs();

    for mut car in car.iter_mut() {
        let y_speed_ratio = (car.speed_y / PLAYER_MAX_SPEED).abs();

        let x_speed_ratio =
            if y_speed_ratio < 0.1 { y_speed_ratio * 2. } else { (((y_speed_ratio - 0.1) / 0.9) * 0.8) + 0.2 };

        if car.fuel > 0. {
            if keyboard_input.pressed(KeyCode::Space) {
                car.speed_y = apply_throttle(car.speed_y, delta);
            } else {
                let brake = PLAYER_BRAKE_RATE * delta;
                if car.speed_y.abs() <= brake {
                    car.speed_y = 0.;
                } else if car.speed_y > 0. {
                    car.speed_y -= brake;
                } else {
                    car.speed_y += brake;
                }
            }
        } else {
            let brake = PLAYER_BRAKE_RATE_NO_FUEL * delta;
            if car.speed_y.abs() <= brake {
                car.speed_y = 0.;
            } else if car.speed_y > 0. {
                car.speed_y -= brake;
            } else {
                car.speed_y += brake;
            }
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            car.speed_x = -x_speed_ratio * PLAYER_MAX_HSPEED;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            car.speed_x = x_speed_ratio * PLAYER_MAX_HSPEED;
        } else {
            car.speed_x = 0.;
        }

        car.y_position += car.speed_y * delta * PLAYER_POSITION_RATIO;
        car.x_position += car.speed_x * delta * PLAYER_POSITION_RATIO;
        // println!("Speed x: {}", car.speed_x);
        // println!("Posit y: {}", car.x_position);
    }
}

pub fn car_collided_with_wall(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut car: Query<(Entity, &mut PlayerOneCar, &Transform, &CollidedWithWall)>) {
    car.iter_mut().for_each(|(id, mut car, transform, collision)| {

        car.speed_x = 0.;
        car.speed_y = 0.;

        if car.fuel > PLAYER_FUEL_LOSS_ON_CRASH * 2. {
            car.fuel -= PLAYER_FUEL_LOSS_ON_CRASH;
        }

        match collision.side {
            CarCollidedSide::Left => car.x_position += 20.,
            CarCollidedSide::Right => car.x_position -= 20.,
        }

        commands.entity(id).despawn();
        commands.spawn((PlayingAll, ToBeRespawned { car: car.clone(), despawn_time: time.elapsed_secs() }));
        spawn_explosion(&mut commands, &asset_server, &mut texture_atlas_layouts, transform.translation);
    });
}

pub fn render_screen(mut car: Query<(&mut PlayerOneCar, &mut Transform)>) {
    car.iter_mut().for_each(|(car, mut transform)| {
        transform.translation.x = car.x_position;
    });
}

pub fn drain_fuel(
    time: Res<Time>,
    playing_data: Res<PlayingData>,
    mut car: Query<&mut PlayerOneCar>,
) {
    if playing_data.race_state != RaceState::Started {
        return;
    }
    let delta = time.delta_secs();
    for mut car in car.iter_mut() {
        car.fuel = calculate_new_fuel(car.fuel, delta);
    }
}

#[inline]
fn apply_throttle(speed_y: f32, delta: f32) -> f32 {
    let ratio = speed_y / PLAYER_MAX_SPEED;
    (speed_y + (1. - ratio) * PLAYER_MAX_ACCEL_RATE * delta).min(PLAYER_MAX_SPEED)
}

#[inline]
fn calculate_new_fuel(fuel: f32, delta: f32) -> f32 {
    (fuel - PLAYER_FUEL_DRAIN_RATE * delta).max(0.)
}

pub fn check_level_complete(
    car: Query<&PlayerOneCar>,
    playing_data: Res<PlayingData>,
    mut next_state: ResMut<NextState<GameGlobalState>>,
) {
    if playing_data.race_state != RaceState::Started {
        return;
    }
    for car in car.iter() {
        if car.fuel > 0.0 && car.y_position >= playing_data.finish_line {
            info!("Level complete! Advancing to next level.");
            next_state.set(GameGlobalState::LevelComplete);
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Speed and y-position sampled every second from the C game at full throttle.
    // t=0 is the first second after the race start semaphore clears.
    // C game has an 8-frame init_delay before acceleration begins, so Rust (no delay,
    // continuous float) will be slightly ahead — tolerances cover that divergence.
    const ACCEL_REFERENCE: &[(u32, f32, f32)] = &[
        //  elapsed_sec    y_px    speed_px_s
        (0,    70.,  174.),
        (1,   339.,  352.),
        (2,   762.,  485.),
        (3,  1299.,  584.),
        (4,  1923.,  659.),
        (5,  2611.,  714.),
        (6,  3347.,  756.),
        (7,  4119.,  787.),
        (8,  4917.,  810.),
        (9,  5736.,  828.),
        (10, 6570.,  840.),
        (11, 7415.,  851.),
        (12, 8268.,  857.),
        (13, 9127.,  862.),
        (14, 9991.,  868.),
        (15,10860.,  870.),
        (16,11730.,  870.),
        (17,12599.,  870.),
        (18,13469.,  870.),
        (19,14338.,  870.),
        (20,15208.,  870.),
    ];

    fn simulate_full_throttle() -> Vec<(f32, f32)> {
        // FPS derived from PLAYER_POSITION_RATIO = ORIGINAL_FPS / 256, so ORIGINAL_FPS = ratio * 256
        let fps = PLAYER_POSITION_RATIO * 256.0;
        let dt = 1.0 / fps;
        let frames_per_sec = fps.round() as u32;
        let mut speed_y = 0.0f32;
        let mut y_traveled = 0.0f32;
        let mut results = Vec::new();
        for _ in 0..ACCEL_REFERENCE.len() {
            for _ in 0..frames_per_sec {
                speed_y = apply_throttle(speed_y, dt);
                y_traveled += speed_y * dt * PLAYER_POSITION_RATIO;
            }
            results.push((speed_y * PLAYER_POSITION_RATIO, y_traveled));
        }
        results
    }

    #[test]
    fn full_throttle_speed_and_position_match_c_reference() {
        // Speed tolerance covers the init_delay diff (≈49 px/s) plus max-speed rounding (≈18 px/s)
        const SPEED_TOLERANCE: f32 = 60.;
        // Y tolerance covers accumulated init_delay effect over 20 seconds (≈453 px peak)
        const Y_TOLERANCE: f32 = 600.;

        let simulated = simulate_full_throttle();
        for (i, &(elapsed_sec, ref_y, ref_speed)) in ACCEL_REFERENCE.iter().enumerate() {
            let (sim_speed, sim_y) = simulated[i];
            let speed_diff = (sim_speed - ref_speed).abs();
            assert!(
                speed_diff <= SPEED_TOLERANCE,
                "t={}s: speed expected≈{} px/s, got {:.1} (diff={:.1})",
                elapsed_sec, ref_speed, sim_speed, speed_diff,
            );
            let y_diff = (sim_y - ref_y).abs();
            assert!(
                y_diff <= Y_TOLERANCE,
                "t={}s: y expected≈{} px, got {:.1} (diff={:.1})",
                elapsed_sec, ref_y, sim_y, y_diff,
            );
        }
    }

    // Fuel sampled every second from the C game during a real race run.
    // Drain is exactly 37 units/s in the C game (integer decrement at ~37 fps).
    // Rust uses PLAYER_FUEL_DRAIN_RATE = 1000/27 ≈ 37.037, so accumulated drift
    // over the full 63-second run is < 3 units — well within the ±5 tolerance.
    const REFERENCE: &[(u32, f32)] = &[
        (0,  2346.), (1,  2309.), (2,  2272.), (3,  2235.), (4,  2198.),
        (5,  2161.), (6,  2124.), (7,  2087.), (8,  2050.), (9,  2013.),
        (10, 1976.), (11, 1939.), (12, 1902.), (13, 1865.), (14, 1828.),
        (15, 1791.), (16, 1754.), (17, 1717.), (18, 1680.), (19, 1643.),
        (20, 1606.), (21, 1569.), (22, 1532.), (23, 1495.), (24, 1458.),
        (25, 1421.), (26, 1384.), (27, 1347.), (28, 1310.), (29, 1273.),
        (30, 1236.), (31, 1199.), (32, 1162.), (33, 1125.), (34, 1088.),
        (35, 1051.), (36, 1014.), (37,  977.), (38,  940.), (39,  903.),
        (40,  866.), (41,  829.), (42,  792.), (43,  755.), (44,  718.),
        (45,  681.), (46,  644.), (47,  607.), (48,  570.), (49,  533.),
        (50,  496.), (51,  459.), (52,  422.), (53,  385.), (54,  348.),
        (55,  311.), (56,  274.), (57,  237.), (58,  200.), (59,  163.),
        (60,  126.), (61,   89.), (62,   52.), (63,   15.),
    ];

    const TOLERANCE: f32 = 5.;

    #[test]
    fn fuel_drain_matches_c_reference() {
        let mut fuel = REFERENCE[0].1;

        for window in REFERENCE.windows(2) {
            let (t0, _)        = window[0];
            let (t1, expected) = window[1];
            let dt = (t1 - t0) as f32;
            fuel = calculate_new_fuel(fuel, dt);
            let diff = (fuel - expected).abs();
            assert!(
                diff <= TOLERANCE,
                "t={}s: expected fuel≈{}, got {:.2} (diff={:.2})",
                t1, expected, fuel, diff,
            );
        }
    }
}
