mod bee;
mod material;
mod behaviours;
mod hive;
mod physcis;
mod enemy;
mod living_creature;

pub use bee::*;
pub use material::*;
use bevy::{prelude::*, sprite::Material2dPlugin};
pub use hive::*;
pub use physcis::*;
pub use behaviours::*;
pub use enemy::*;
pub use living_creature::*;

use self::behaviours::BehaviourPlugin;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<UniversalMaterial>::default());

        app.init_resource::<HiveBuildings>();

        app.add_systems(PreUpdate, update_bee_material_system);
        app.add_systems(PreUpdate, update_wasp_material_system);
        app.add_systems(PreUpdate, update_buildings_system);
        app.add_systems(PreUpdate, prepare_atlases_system);

        app.add_systems(
            PostUpdate,
            (
                move_to_target_system,
                collision_system.after(move_to_target_system),
                integration_system.after(collision_system),
                orientation_system.after(integration_system),
            ),
        );

        app.add_plugins(BehaviourPlugin);
    }
}
