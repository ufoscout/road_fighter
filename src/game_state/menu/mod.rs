use bevy::prelude::*;
use components::*;
use constants::MENU_ARROW_BASE_Y_OFFEST;
use resources::MenuData;

use super::GameGlobalState;

mod components;
mod constants;
mod resources;
mod systems;

/// The plugin that sets up the disclaimer screen
pub struct MenuStatePlugin;

impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameGlobalState::Menu), on_enter)
            .add_systems(OnExit(GameGlobalState::Menu), on_exit)
            .init_resource::<MenuData>()
            .add_systems(
                Update,
                (systems::handle_key_pressed, systems::render_arrow).run_if(in_state(GameGlobalState::Menu)),
            );
    }
}

fn on_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the title
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/title.png"),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 80.0, 0.0)),
        MenuAll,
        MenuTitle,
    ));

    // Spawn the arrow
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/arrow.png"),
            ..default()
        },
        Transform::from_translation(Vec3::new(-110.0, MENU_ARROW_BASE_Y_OFFEST, 1.0)),
        MenuAll,
        MenuArrow,
    ));

    // Add menu entries
    {
        let font = asset_server.load("fonts/tanglewo.ttf");

        let options = r#"ONE PLAYER
TWO PLAYERS
OPTIONS
EXIT"#;

        commands.spawn((
            Text2d::new(options),
            TextFont { font, font_size: 30.0, ..default() },
            TextLayout::new_with_justify(Justify::Left),
            Transform::from_translation(Vec3::new(25.0, -75.0, 0.0)),
            MenuAll,
        ));
    }
}

// Despawn the menu screen
fn on_exit(mut commands: Commands, menu_all: Query<(Entity, &MenuAll)>) {
    menu_all.iter().for_each(|(entity, _)| commands.entity(entity).despawn());
}
