use core::{
    spawn_hive_visual, AppState, Bee, BeeKind, CorePlugin, HiveGraph, HiveMap,
    MoveToNavigationTargetBehaviour, NavigationTarget, Velocity, VelocityOriented,
};

use bevy::{
    prelude::*,
    render::mesh::shape::Quad,
    sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle},
    utils::petgraph::visit::EdgeRef,
};
use rand::Rng;

pub mod core;
pub mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CorePlugin);

    app.add_systems(Startup, camera_setup);

    if utils::is_local_build() {
        app.add_systems(Startup, setup);
        app.add_systems(Update, spawn_bees);
    } else {
    }

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(1.0 / 0.1)),
        ..default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut clear_color: ResMut<ClearColor>,
) {
    next_state.set(AppState::InGame);

    spawn_hive_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );

    clear_color.0 = Color::WHITE;
}
