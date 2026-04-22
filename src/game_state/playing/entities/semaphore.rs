use bevy::prelude::*;

use crate::{
    constants::WINDOW_HEIGHT,
    game_state::{
        playing::{PlayingAll, PlayingData, PlayingMap},
        GameGlobalState,
    },
};

// Duration of each semaphore phase (blank → red1 → blank → red2 → blank → red3 → blank → green)
const SEMAPHORE_STEP_SECS: f32 = 1.;
pub const SEMAPHORE_TOTAL_SECS: f32 = SEMAPHORE_STEP_SECS * 7.0;

/// Counts down from SEMAPHORE_TOTAL_SECS to 0.
/// While positive, the player car cannot accelerate.
#[derive(Resource, Default)]
pub struct SemaphoreCountdown(pub f32);

impl SemaphoreCountdown {
    pub fn is_active(&self) -> bool {
        self.0 > 0.0
    }
}

#[derive(Component)]
struct Semaphore {
    elapsed: f32,
    tile_rects: [Rect; 5],
}

pub struct SemaphorePlugin;

impl Plugin for SemaphorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SemaphoreCountdown>().add_systems(
            Update,
            semaphore_animation_system.run_if(in_state(GameGlobalState::Playing)),
        );
    }
}

pub fn spawn_semaphore(
    playing_data: &PlayingData,
    commands: &mut Commands,
    asset_server: &AssetServer,
    semaphore_countdown: &mut SemaphoreCountdown,
) {
    let map_data = playing_data.level.map_data();

    let Some((sem_x, sem_y)) = map_data.semaphore_position else {
        return;
    };

    let tiles: [_; 5] = core::array::from_fn(|i| {
        let [bank, num] = map_data.semaphore_tiles[i];
        &map_data.tiles[bank][num]
    });

    let image_path = tiles[0].tile_source.clone();
    let tile_w = tiles[0].width as f32;
    let tile_h = tiles[0].height as f32;

    let tile_rects: [Rect; 5] = core::array::from_fn(|i| {
        let t = tiles[i];
        Rect::new(t.x as f32, t.y as f32, (t.x + t.width) as f32, (t.y + t.height) as f32)
    });

    let world_x = sem_x - map_data.width / 2.0 + tile_w / 2.0;
    let world_y = WINDOW_HEIGHT / 2.0 - sem_y - tile_h / 2.0;

    commands.spawn((
        Sprite { image: asset_server.load(image_path), rect: Some(tile_rects[0]), ..default() },
        Transform::from_translation(Vec3::new(world_x, world_y, 10.0)),
        PlayingAll,
        PlayingMap { y_position: world_y },
        Semaphore { elapsed: 0.0, tile_rects },
    ));

    semaphore_countdown.0 = SEMAPHORE_TOTAL_SECS;
}

fn semaphore_animation_system(
    time: Res<Time>,
    mut countdown: ResMut<SemaphoreCountdown>,
    mut query: Query<(&mut Semaphore, &mut Sprite)>,
) {
    if !countdown.is_active() {
        return;
    }

    let delta = time.delta_secs();
    countdown.0 = (countdown.0 - delta).max(0.0);

    for (mut sem, mut sprite) in query.iter_mut() {
        sem.elapsed += delta;
        sprite.rect = Some(sem.tile_rects[tile_index(sem.elapsed)]);
    }
}

// Mirrors the C animation sequence: tile 0 = blank/off, 1-3 = red lights, 4 = green
fn tile_index(elapsed: f32) -> usize {
    if elapsed >= SEMAPHORE_STEP_SECS * 7.0 {
        4 // green — go!
    } else if elapsed >= SEMAPHORE_STEP_SECS * 6.0 {
        0 // blank
    } else if elapsed >= SEMAPHORE_STEP_SECS * 5.0 {
        3 // third red
    } else if elapsed >= SEMAPHORE_STEP_SECS * 4.0 {
        0 // blank
    } else if elapsed >= SEMAPHORE_STEP_SECS * 3.0 {
        2 // second red
    } else if elapsed >= SEMAPHORE_STEP_SECS * 2.0 {
        0 // blank
    } else if elapsed >= SEMAPHORE_STEP_SECS {
        1 // first red
    } else {
        0 // blank
    }
}
