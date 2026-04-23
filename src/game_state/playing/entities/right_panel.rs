use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use crate::{
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    game_state::{
        playing::{
            constants::{PLAYER_MAX_FUEL, PLAYER_MAX_SPEED},
            FuelBar, PlayerOneCar, PlayingAll, SpeedBar,
        },
        GameGlobalState,
    },
};

// Scoreboard panel dimensions (pixels, SDL top-left origin)
const SCOREBOARD_WIDTH: f32 = 145.;
const SCOREBOARD_LEFT_SCREEN_X: f32 = WINDOW_WIDTH - SCOREBOARD_WIDTH; // 367

// Bar parameters (SDL coordinates)
const BAR_WIDTH: f32 = 32.;
const BAR_MAX_HEIGHT: f32 = 112.;
const BAR_BOTTOM_SCREEN_Y: f32 = 367.;
const SPEED_BAR_X_OFFSET: f32 = 28.; // from scoreboard left
const FUEL_BAR_X_OFFSET: f32 = 76.;  // from scoreboard left

// Bevy world-space conversions: world_x = screen_x - W/2, world_y = H/2 - screen_y
const SCOREBOARD_CENTER_X: f32 =
    SCOREBOARD_LEFT_SCREEN_X + SCOREBOARD_WIDTH / 2. - WINDOW_WIDTH / 2.;

const SPEED_BAR_CENTER_X: f32 =
    SCOREBOARD_LEFT_SCREEN_X + SPEED_BAR_X_OFFSET + BAR_WIDTH / 2. - WINDOW_WIDTH / 2.;

const FUEL_BAR_CENTER_X: f32 =
    SCOREBOARD_LEFT_SCREEN_X + FUEL_BAR_X_OFFSET + BAR_WIDTH / 2. - WINDOW_WIDTH / 2.;

// Bottom of the bars in world space
const BAR_BOTTOM_Y: f32 = WINDOW_HEIGHT / 2. - BAR_BOTTOM_SCREEN_Y; // -175

const PANEL_Z: f32 = 500.;

pub struct RightPanelPlugin;

impl Plugin for RightPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameGlobalState::Playing), spawn_right_panel)
            .add_systems(Update, update_bars.run_if(in_state(GameGlobalState::Playing)));
    }
}

fn spawn_right_panel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    // Scoreboard background
    commands.spawn((
        Sprite {
            image: asset_server.load("graphics/scoreboard.png"),
            ..default()
        },
        Transform::from_translation(Vec3::new(SCOREBOARD_CENTER_X, 0., PANEL_Z)),
        PlayingAll,
    ));

    let white = images.add(Image::new(
        Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
        TextureDimension::D2,
        vec![255u8, 255, 255, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    ));

    // Speed bar
    commands.spawn((
        Sprite {
            image: white.clone(),
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BAR_WIDTH, 1.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(SPEED_BAR_CENTER_X, BAR_BOTTOM_Y, PANEL_Z + 1.)),
        Visibility::Hidden,
        PlayingAll,
        SpeedBar,
    ));

    // Fuel bar
    commands.spawn((
        Sprite {
            image: white,
            color: Color::WHITE,
            custom_size: Some(Vec2::new(BAR_WIDTH, 1.)),
            ..default()
        },
        Transform::from_translation(Vec3::new(FUEL_BAR_CENTER_X, BAR_BOTTOM_Y, PANEL_Z + 1.)),
        Visibility::Hidden,
        PlayingAll,
        FuelBar,
    ));
}

fn update_bars(
    car: Query<&PlayerOneCar>,
    mut speed_bar: Query<(&mut Sprite, &mut Transform, &mut Visibility), (With<SpeedBar>, Without<FuelBar>)>,
    mut fuel_bar: Query<(&mut Sprite, &mut Transform, &mut Visibility), (With<FuelBar>, Without<SpeedBar>)>,
) {
    let (speed_ratio, fuel_ratio) = car
        .iter()
        .next()
        .map(|c| {
            (
                (c.speed_y / PLAYER_MAX_SPEED).clamp(0., 1.),
                (c.fuel / PLAYER_MAX_FUEL).clamp(0., 1.),
            )
        })
        .unwrap_or((0., 0.));

    set_bar(&mut speed_bar, speed_ratio * BAR_MAX_HEIGHT);
    set_bar(&mut fuel_bar, fuel_ratio * BAR_MAX_HEIGHT);
}

fn set_bar(
    query: &mut Query<(&mut Sprite, &mut Transform, &mut Visibility), impl bevy::ecs::query::QueryFilter>,
    height: f32,
) {
    for (mut sprite, mut transform, mut visibility) in query.iter_mut() {
        if height < 1. {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Inherited;
            sprite.custom_size = Some(Vec2::new(BAR_WIDTH, height));
            transform.translation.y = BAR_BOTTOM_Y + height / 2.;
        }
    }
}
