use bevy::prelude::*;

use crate::game_state::GameGlobalState;

use super::{
    components::*,
    resources::{IntroductionData, IntroductionStep},
};

/// Move to menu screen whatever key is pressed
pub fn handle_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    mut introduction_state: ResMut<IntroductionData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.get_just_pressed().next().is_some() {
        introduction_state.step = introduction_state.step.next();
        if introduction_state.step == IntroductionStep::End {
            next_state.set(GameGlobalState::Menu);
        }
    }
}

/// Move to menu screen whatever key is pressed
pub fn render_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    introduction_state: Res<IntroductionData>,
    background: Query<(Entity, &IntroductionBackground)>,
) {
    background.iter().for_each(|(entity, _)| {
        commands.entity(entity).despawn();
    });

    match introduction_state.step {
        IntroductionStep::StepOne => {
            commands.spawn((
                SpriteBundle { texture: asset_server.load("graphics/retroremakes.png"), ..default() },
                IntroductionAll,
                IntroductionBackground,
            ));
        }
        IntroductionStep::StepTwo => {
            commands.spawn((
                SpriteBundle { texture: asset_server.load("graphics/konami2.png"), ..default() },
                IntroductionAll,
                IntroductionBackground,
            ));
        }
        IntroductionStep::End => {}
    }
}
