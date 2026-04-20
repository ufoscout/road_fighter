use bevy::prelude::*;

use crate::game_state::{menu::resources::MenuEntry, GameGlobalState};

use super::{
    constants::{MENU_ARROW_BASE_Y_OFFEST, MENU_ARROW_Y_STEP},
    resources::MenuData,
    MenuArrow,
};

/// The system that handles key presses in the menu
pub fn handle_key_pressed(
    mut next_state: ResMut<NextState<GameGlobalState>>,
    mut menu_data: ResMut<MenuData>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        menu_data.selected_entry = menu_data.selected_entry.next()
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        menu_data.selected_entry = menu_data.selected_entry.previous()
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        println!("Selected entry: {:?}", menu_data.selected_entry);

        if menu_data.selected_entry == MenuEntry::Exit {
            // Exit the game if the selected entry is Exit
            exit.write(AppExit::Success);
        } else if menu_data.selected_entry == MenuEntry::OnePlayer {
            // Set the next state to Playing if the selected entry is OnePlayer
            next_state.set(GameGlobalState::Playing);
        }
    }
}

/// The system that moves the cursor to the selected entry
pub fn render_arrow(menu_data: Res<MenuData>, mut query: Query<(&mut Transform, &MenuArrow)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.translation.y =
            menu_data.selected_entry.index() as f32 * MENU_ARROW_Y_STEP + MENU_ARROW_BASE_Y_OFFEST;
    }
}
