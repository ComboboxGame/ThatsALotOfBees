mod bee;
mod bee_material;
mod behaviours;
mod hive;
mod movement;
mod wasp;

pub use bee::*;
pub use bee_material::*;
use bevy::{prelude::*, sprite::Material2dPlugin};
pub use hive::*;
pub use movement::*;
pub use wasp::*;

use self::behaviours::BehaviourPlugin;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<BeeMaterial>::default());

        app.init_resource::<HiveBuildings>();

        app.add_systems(PreUpdate, update_bee_material_system);
        app.add_systems(PreUpdate, update_wasp_material_system);
        app.add_systems(PreUpdate, update_buildings_system);
        app.add_systems(PreUpdate, prepare_atlases_system);

        app.add_systems(
            PostUpdate,
            (
                movement_system,
                movement_orientation_system.after(movement_system),
            ),
        );

        app.add_plugins(BehaviourPlugin);
    }
}
