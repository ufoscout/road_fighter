use bevy::{camera::{ScalingMode, Viewport}, log::LogPlugin, prelude::*, window::WindowResolution};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game_state::{
    disclaimer::DisclaimerStatePlugin, introduction::IntroductionStatePlugin,
    level_complete::LevelCompletePlugin, menu::MenuStatePlugin, playing::PlayingStatePlugin,
    GameGlobalState,
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
        .add_plugins(LevelCompletePlugin)
        .run();
}

struct MainWindowPlugin;

impl Plugin for MainWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(PostUpdate, update_letterbox_viewport)
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

fn update_letterbox_viewport(
    windows: Query<&Window>,
    mut camera_query: Query<&mut Camera>,
    mut prev_size: Local<UVec2>,
) {
    let Ok(window) = windows.single() else { return };
    let size = window.physical_size();
    if size == *prev_size || size.x == 0 || size.y == 0 {
        return;
    }
    *prev_size = size;

    let Ok(mut camera) = camera_query.single_mut() else { return };

    let target_ratio = WINDOW_WIDTH / WINDOW_HEIGHT;
    let window_ratio = size.x as f32 / size.y as f32;

    let (vp_w, vp_h) = if window_ratio > target_ratio {
        // Window wider than target: pillarbox (bars on left/right)
        ((size.y as f32 * target_ratio) as u32, size.y)
    } else {
        // Window taller than target: letterbox (bars on top/bottom)
        (size.x, (size.x as f32 / target_ratio) as u32)
    };

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new((size.x - vp_w) / 2, (size.y - vp_h) / 2),
        physical_size: UVec2::new(vp_w, vp_h),
        ..default()
    });
}
