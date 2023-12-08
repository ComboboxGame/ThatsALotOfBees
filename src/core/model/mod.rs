mod bee;
mod behaviours;
mod enemy;
mod buildings;
mod living_creature;
mod material;
mod physcis;

pub use bee::*;
pub use behaviours::*;
use bevy::{sprite::Material2dPlugin, render::mesh::shape::Quad};
pub use enemy::*;
pub use buildings::*;
pub use living_creature::*;
pub use material::*;
pub use physcis::*;

use self::behaviours::BehaviourPlugin;

pub const BEE_MESH: Handle<Mesh> = Handle::weak_from_u128(1311196983320128547);
pub const WASP_MESH: Handle<Mesh> = Handle::weak_from_u128(1311196983120126547);
pub const BIRB_MESH: Handle<Mesh> = Handle::weak_from_u128(1311196983520121547);

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<UniversalMaterial>::default());

        app.init_resource::<HiveBuildings>();

        app.add_systems(Startup, create_meshes);

        app.add_systems(PreUpdate, update_bee_material_system);
        app.add_systems(PreUpdate, update_wasp_material_system);
        app.add_systems(PreUpdate, update_buildings_system);
        app.add_systems(PreUpdate, prepare_atlases_system);

        app.add_systems(Update, living_creature_system);
        app.add_systems(Update, buildings_system);

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

pub fn create_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
) {
    meshes.insert(BEE_MESH, Quad::new(Vec2::splat(24.0)).into());
    meshes.insert(WASP_MESH, Quad::new(Vec2::splat(24.0)).into());
    meshes.insert(BIRB_MESH, Quad::new(Vec2::splat(24.0)).into());
}
