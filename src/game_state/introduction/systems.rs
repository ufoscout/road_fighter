
use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::{components::*, state::IntroductionState};

/// Move to menu screen whatever key is pressed
pub fn introduction_key_pressed(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameGlobalState>>, mut introduction_state: ResMut<IntroductionState>, background: Query<(Entity, &IntroductionBackground)>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.get_just_pressed().next().is_some() {

        background.iter().for_each(|(entity, _)| {
            commands.entity(entity).despawn();
        });

        match introduction_state.step {
            0 => {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("graphics/konami2.png"),
                        ..default()
                    },
                    IntroductionBackground
                ));
            },
            _ => {
                // Move to the next step
                next_state.set(GameGlobalState::Menu);
            }
        }
        introduction_state.step += 1;
    }
}
