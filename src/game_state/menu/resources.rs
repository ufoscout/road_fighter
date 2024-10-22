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

impl MenuEntry {
    pub fn next(&self) -> Self {
        match self {
            Self::OnePlayer => Self::TwoPlayers,
            Self::TwoPlayers => Self::Options,
            Self::Options => Self::Exit,
            Self::Exit => Self::OnePlayer,
        }
    }

    pub fn previous(&self) -> Self {
        match self {
            Self::OnePlayer => Self::Exit,
            Self::TwoPlayers => Self::OnePlayer,
            Self::Options => Self::TwoPlayers,
            Self::Exit => Self::Options,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::OnePlayer => 0,
            Self::TwoPlayers => 1,
            Self::Options => 2,
            Self::Exit => 3,
        }
    }
}
