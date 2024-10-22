use bevy::{prelude::*, render::camera::ScalingMode, window::WindowResolution};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game_state::{disclaimer::DisclaimerStatePlugin, introduction::IntroductionStatePlugin, GameGlobalState};

mod constants;
mod game_state;

fn main() {
    App::new()
        // Plugin that sets up the main window
        .add_plugins(MainWindowPlugin)
        // Set up the global game state
        .init_state::<GameGlobalState>()
        .add_plugins(DisclaimerStatePlugin)
        .add_plugins(IntroductionStatePlugin)
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
                    resizable: true,
                    ..default()
                }),
                ..default()
            }));
    }
}

fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            // If the window is resized, the camera will automatically adjust
            scaling_mode: ScalingMode::AutoMin {
                min_width: WINDOW_WIDTH,
                min_height: WINDOW_HEIGHT,
            },
            far: 1000.,
            near: -1000.,            
            ..default()
        },
        ..default()
    });
}
