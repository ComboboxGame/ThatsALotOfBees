mod bee;
mod behaviours;
mod buildings;
mod currency;
mod enemy;
mod living_creature;
mod material;
mod physcis;

pub use bee::*;
pub use behaviours::*;
use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Material2dPlugin, utils::HashMap};
pub use buildings::*;
pub use currency::*;
pub use enemy::*;
pub use living_creature::*;
pub use material::*;
pub use physcis::*;

use crate::core::spawn_hive_visual;

use self::behaviours::BehaviourPlugin;

use super::AppState;

pub const BEE_MESH: Handle<Mesh> = Handle::weak_from_u128(1311196983320128547);
pub const WASP_MESH: Handle<Mesh> = Handle::weak_from_u128(1311196983120126547);
pub const BIRB_MESH: Handle<Mesh> = Handle::weak_from_u128(1311196983520121547);

pub struct ModelPlugin;

impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<UniversalMaterial>::default());
        app.add_plugins(Material2dPlugin::<BuildingMaterial>::default());

        app.init_resource::<HiveBuildings>();
        app.init_resource::<CurrencyStorage>();

        app.add_systems(Startup, create_meshes);

        app.add_systems(PreUpdate, update_bee_material_system);
        app.add_systems(PreUpdate, update_wasp_material_system);
        app.add_systems(PreUpdate, update_buildings_system);
        app.add_systems(PreUpdate, prepare_atlases_system);

        app.add_systems(Update, gain_system);
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

        app.add_systems(Last, (
            entered_main_menu.run_if(state_changed::<AppState>().and_then(in_state(AppState::MainMenu))),
            enetered_game.run_if(state_changed::<AppState>().and_then(in_state(AppState::InGame))),
        ));

        app.add_plugins(BehaviourPlugin);
    }
}

pub fn create_meshes(mut meshes: ResMut<Assets<Mesh>>) {
    meshes.insert(BEE_MESH, Quad::new(Vec2::splat(24.0)).into());
    meshes.insert(WASP_MESH, Quad::new(Vec2::splat(24.0)).into());
    meshes.insert(BIRB_MESH, Quad::new(Vec2::splat(24.0)).into());
}


fn entered_main_menu() {

}

fn enetered_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    mut hive_buildings: ResMut<HiveBuildings>,
    mut currency: ResMut<CurrencyStorage>,
) {
    *hive_buildings = HiveBuildings::default();
    *currency = CurrencyStorage::default();

    commands.spawn(BeeBundle::from((BeeType::Queen, get_building_position(8))));
    
    spawn_hive_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );
}
