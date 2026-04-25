use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game_state::{
    playing::{
        constants::{PLAYER_FUEL_RECHARGE, PLAYER_MAX_FUEL},
        PlayerOneCar, PlayingAll, PlayingMap,
        resources::{PlayingData, RaceState},
    },
    GameGlobalState,
};

use super::map::wall_distances;

// Player car's fixed screen (world) y — matches spawn_player_car's Transform
const PLAYER_WORLD_Y: f32 = -126.0;
// How far above the player (in screen/world units) hearts spawn — slightly off-screen top
const SPAWN_ABOVE: f32 = 350.0;
// Player must travel this many map-space units between spawns (~1 heart per 5-6 seconds at speed)
const SPAWN_INTERVAL: f32 = 5000.0;
// Collection radius in screen space
const COLLECT_RADIUS_Y: f32 = 20.0;
const COLLECT_RADIUS_X: f32 = 24.0;

#[derive(Component)]
pub struct FuelHeart;

#[derive(Resource, Default)]
struct FuelHeartSpawner {
    distance_since_last_spawn: f32,
    last_y_position: Option<f32>,
}

pub struct FuelHeartPlugin;

impl Plugin for FuelHeartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FuelHeartSpawner>()
            .add_systems(OnEnter(GameGlobalState::Playing), reset_spawner)
            .add_systems(
                Update,
                (spawn_fuel_hearts, collect_fuel_hearts, despawn_fuel_hearts)
                    .run_if(in_state(GameGlobalState::Playing)),
            );
    }
}

fn reset_spawner(mut spawner: ResMut<FuelHeartSpawner>) {
    *spawner = FuelHeartSpawner::default();
}

fn spawn_fuel_hearts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawner: ResMut<FuelHeartSpawner>,
    playing_data: Res<PlayingData>,
    car: Query<&PlayerOneCar>,
    spatial_query: SpatialQuery,
) {
    if playing_data.race_state != RaceState::Started {
        return;
    }
    let Ok(car) = car.single() else { return };

    let delta = match spawner.last_y_position {
        Some(last) => (car.y_position - last).max(0.0),
        None => 0.0,
    };
    spawner.last_y_position = Some(car.y_position);
    spawner.distance_since_last_spawn += delta;

    if spawner.distance_since_last_spawn < SPAWN_INTERVAL {
        return;
    }
    spawner.distance_since_last_spawn = 0.0;

    // The heart's initial screen y is PLAYER_WORLD_Y + SPAWN_ABOVE (just above the viewport).
    // Because world_y = map_y - scroll.y = map_y - car.y_position, we get:
    //   map_y = car.y_position + PLAYER_WORLD_Y + SPAWN_ABOVE
    let initial_world_y = PLAYER_WORLD_Y + SPAWN_ABOVE;
    let map_y = car.y_position + initial_world_y;

    // Ray-cast left and right from the player's x at the spawn screen-y to find the road center
    let probe = Transform::from_translation(Vec3::new(car.x_position, initial_world_y, 0.0));
    let road_center_x = wall_distances(&probe, &spatial_query)
        .map(|(left_dist, right_dist)| car.x_position + (right_dist - left_dist) / 2.0)
        .unwrap_or(car.x_position);

    commands.spawn((
        Sprite { image: asset_server.load("graphics/fuel.png"), ..default() },
        Transform::from_translation(Vec3::new(road_center_x, initial_world_y, 5.0)),
        PlayingAll,
        PlayingMap { y_position: map_y },
        FuelHeart,
    ));
}

fn collect_fuel_hearts(
    mut commands: Commands,
    hearts: Query<(Entity, &PlayingMap, &Transform), With<FuelHeart>>,
    mut cars: Query<&mut PlayerOneCar>,
) {
    let Ok(mut car) = cars.single_mut() else { return };

    for (entity, map, transform) in hearts.iter() {
        // world_y of the heart relative to the player's fixed screen position
        let heart_world_y = map.y_position - car.y_position;
        let dy = (heart_world_y - PLAYER_WORLD_Y).abs();
        let dx = (transform.translation.x - car.x_position).abs();
        if dy < COLLECT_RADIUS_Y && dx < COLLECT_RADIUS_X {
            car.fuel = (car.fuel + PLAYER_FUEL_RECHARGE).min(PLAYER_MAX_FUEL);
            commands.entity(entity).despawn();
        }
    }
}

fn despawn_fuel_hearts(
    mut commands: Commands,
    hearts: Query<(Entity, &PlayingMap), With<FuelHeart>>,
    cars: Query<&PlayerOneCar>,
) {
    let Ok(car) = cars.single() else { return };
    for (entity, map) in hearts.iter() {
        let heart_world_y = map.y_position - car.y_position;
        if heart_world_y < PLAYER_WORLD_Y - 50.0 {
            commands.entity(entity).despawn();
        }
    }
}
