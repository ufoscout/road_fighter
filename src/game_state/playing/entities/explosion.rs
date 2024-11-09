use bevy::prelude::*;

use crate::game_state::{playing::{Explosion, ExplosionTimer}, GameGlobalState};

const EXPLOSION_LEN: usize = 12;

/// The plugin that handles the player car explosion
pub struct PlayerCarExplosionPlugin;

impl Plugin for PlayerCarExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (explosion_animation_system).run_if(in_state(GameGlobalState::Playing)),
        );
    }
}


pub fn spawn_explosion(
	commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    translation: Vec3,
) {

		// spawn the explosion sprite
		commands
			.spawn((
				SpriteBundle {
					texture: asset_server.load("graphics/explosion.png"),
					transform: Transform {
						translation,
						..Default::default()
					},
					..Default::default()
				},
				TextureAtlas {
					layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(64, 64), 1, EXPLOSION_LEN as u32, None, None)),
					index: 0,
				},
			))
			.insert(Explosion)
			.insert(ExplosionTimer::default());

}

fn explosion_animation_system(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlas), With<Explosion>>,
) {
	for (entity, mut timer, mut sprite) in &mut query {
		timer.0.tick(time.delta());
		if timer.0.finished() {
			sprite.index += 1; // move to next sprite cell
			if sprite.index >= EXPLOSION_LEN {
				commands.entity(entity).despawn();
			}
		}
	}
}