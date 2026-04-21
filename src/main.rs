use bevy::{camera::ScalingMode, log::LogPlugin, prelude::*, window::WindowResolution};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game_state::{
    disclaimer::DisclaimerStatePlugin, introduction::IntroductionStatePlugin, menu::MenuStatePlugin,
    playing::PlayingStatePlugin, GameGlobalState,
};

mod chunk_manager;
mod constants;
mod error;
mod game_state;

fn main() {
    App::new()
        // Plugin that sets up the main window
        .add_plugins(MainWindowPlugin)
        // Set up the global game state
        .init_state::<GameGlobalState>()
        .add_plugins(DisclaimerStatePlugin)
        .add_plugins(IntroductionStatePlugin)
        .add_plugins(MenuStatePlugin)
        .add_plugins(PlayingStatePlugin)
        .run();
}

struct MainWindowPlugin;

impl Plugin for MainWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems that run once at the start of the app
            .add_systems(Startup, setup)
            // Set up the main window
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Road Fighter".to_string(),
                            resolution: WindowResolution::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
                                .with_scale_factor_override(2.),
                            resizable: true,
                            ..default()
                        }),
                        ..default()
                    })
                    // Setting the texture filtering mode to Nearest (and not Linear) makes the pixels to appear crisp instead of blurry
                    // .set(ImagePlugin::default_nearest())
                    .set(LogPlugin {
                        filter: "warn,road_fighter=debug".into(),
                        level: bevy::log::Level::WARN,
                        ..default()
                    }),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin { min_width: WINDOW_WIDTH, min_height: WINDOW_HEIGHT },
            far: 1000.,
            near: -1000.,
            ..OrthographicProjection::default_2d()
        }),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..Default::default()
        },
        Msaa::Off,
    ));
}
