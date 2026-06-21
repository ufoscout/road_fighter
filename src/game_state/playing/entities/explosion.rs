use bevy::prelude::*;

use crate::game_state::{playing::{Explosion, ExplosionTimer, PlayingAll}, GameGlobalState};

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
				Sprite {
					image: asset_server.load("graphics/explosion.png"),
					texture_atlas: Some(TextureAtlas {
						layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(64, 64), 1, EXPLOSION_LEN as u32, None, None)),
						index: 0,
					}),
					..Default::default()
				},
				Transform {
					translation,
					..Default::default()
				},
				PlayingAll,
			))
			.insert(Explosion)
			.insert(ExplosionTimer::default());

}

fn explosion_animation_system(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut ExplosionTimer, &mut Sprite), With<Explosion>>,
) {
	for (entity, mut timer, mut sprite) in &mut query {
		timer.0.tick(time.delta());
		if timer.0.just_finished() {
			if let Some(atlas) = &mut sprite.texture_atlas {
				atlas.index += 1;
				if atlas.index >= EXPLOSION_LEN {
					commands.entity(entity).despawn();
				}
			}
		}
	}
}