use bevy::{input::InputPlugin, prelude::*, sprite::Material2dPlugin};

mod builder;
mod camera;
mod input;
mod model;
mod navigation;
mod ui;

pub use builder::*;
pub use input::*;
pub use model::*;
pub use navigation::*;
pub use ui::*;

use self::{camera::in_game_camera_system, navigation::NavigationPlugin};

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

        app.add_plugins(InputHelperPlugin);
        app.add_plugins(NavigationPlugin);
        app.add_plugins(UiPlugin);
        app.add_plugins(ModelPlugin);

        app.add_systems(
            Update,
            in_game_camera_system.run_if(in_state(AppState::InGame)),
        );
    }
}
