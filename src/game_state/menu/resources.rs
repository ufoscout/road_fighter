use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MenuData {
    pub selected_entry: MenuEntry,
}

#[derive(Debug, PartialEq, Default)]
pub enum MenuEntry {
    #[default]
    OnePlayer,
    TwoPlayers,
    Options,
    Exit,
}
