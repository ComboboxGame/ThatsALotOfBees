mod bee;
mod behaviours;
mod currency;
mod enemy;
mod hive;
mod living_creature;
mod material;
mod physcis;

pub use bee::*;
pub use behaviours::*;
use bevy::{prelude::*, sprite::Material2dPlugin, utils::HashMap};
pub use currency::*;
pub use enemy::*;
pub use hive::*;
pub use living_creature::*;
pub use material::*;
pub use physcis::*;

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<UniversalMaterial>::default());

        app.init_resource::<HiveBuildings>();
        app.init_resource::<CurrencyStorage>();

        app.add_systems(PreUpdate, update_bee_material_system);
        app.add_systems(PreUpdate, update_wasp_material_system);
        app.add_systems(PreUpdate, update_buildings_system);
        app.add_systems(PreUpdate, prepare_atlases_system);
        app.add_systems(PreUpdate, earn_currency);

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
