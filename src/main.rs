use bevy::{prelude::*, window::WindowResolution};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game_state::{disclaimer::DisclaimerPlugin, Game};

mod constants;
mod game_state;

fn main() {
    App::new()
        // Add a global resource that holds the game state
        .init_resource::<Game>()

        .add_plugins(DisclaimerPlugin)
        
        // Plugin that sets up the main window
        .add_plugins(MainWindowPlugin)
        .run();
}

struct MainWindowPlugin;

impl Plugin for MainWindowPlugin {

    fn build(&self, app: &mut App) {
        app
        // Systems that run once at the start of the app
        .add_systems(Startup, setup)

        // Set up the main window
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Road Fighter".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }));
    }

}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());
}