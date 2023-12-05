mod bee;
mod bee_material;
mod hive;
mod movement;

pub use bee::*;
pub use bee_material::*;
use bevy::{prelude::*, sprite::Material2dPlugin};
pub use hive::*;
pub use movement::*;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<BeeMaterial>::default());

        app.init_resource::<HiveBuildings>();

        app.add_systems(PreUpdate, update_bee_material_system);
        app.add_systems(PreUpdate, update_buildings_system);

        app.add_systems(
            PostUpdate,
            (
                movement_system,
                movement_orientation_system.after(movement_system),
            ),
        );
    }
}
