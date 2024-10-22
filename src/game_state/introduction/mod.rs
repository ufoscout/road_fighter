use bevy::prelude::*;
use resources::IntroductionData;

use super::GameGlobalState;

mod components;
mod resources;
mod systems;

/// The plugin that sets up the disclaimer screen
pub struct IntroductionStatePlugin;

impl Plugin for IntroductionStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IntroductionData>()
            .add_systems(OnEnter(GameGlobalState::Introduction), on_enter)
            .add_systems(
                Update,
                (systems::handle_key_pressed, systems::render_screen).run_if(in_state(GameGlobalState::Introduction)),
            );
    }
}

fn on_enter(mut intro_state: ResMut<IntroductionData>) {
    *intro_state = Default::default();
}
