use bevy::prelude::*;
use plugin::MainWindowPlugin;

mod constants;
mod plugin;
mod setup;

 
fn main() {
    App::new()
    // System that runs once at the start of the app
    .add_systems(Startup, setup::setup)

    // Plugin that sets up the main window
     .add_plugins(MainWindowPlugin).run();
}
