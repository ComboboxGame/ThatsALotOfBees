use bevy::prelude::*;

mod physics;

pub struct CorePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
    }
}
