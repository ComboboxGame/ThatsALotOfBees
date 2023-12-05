use bevy::{prelude::*, sprite::Material2dPlugin};

mod builder;
mod camera;
mod model;
mod navigation;
mod ui;

pub use builder::*;
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

        app.add_plugins(NavigationPlugin);
        app.add_plugins(UIPlugin);
        
        app.add_plugins(UiMaterialPlugin::<BeeMaterial>::default());
        app.add_plugins(Material2dPlugin::<BeeMaterial>::default());

        app.add_systems(PreUpdate, update_bee_material_system);
        app.add_systems(Update, navigation_system);
        app.add_systems(Update, navigated_movement_system.after(navigation_system));

        app.add_systems(PostUpdate, movement_system);
        app.add_systems(
            PostUpdate,
            movement_orientation_system.after(movement_system),
        );

        app.add_systems(
            Update,
            in_game_camera_system.run_if(in_state(AppState::InGame)),
        );
    }
}
