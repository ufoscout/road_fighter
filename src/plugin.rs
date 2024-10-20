use bevy::{prelude::*, window::WindowResolution};
 
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
 
pub struct MainWindowPlugin;
 
impl Plugin for MainWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
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