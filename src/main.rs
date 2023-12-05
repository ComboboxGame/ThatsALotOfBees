use core::{spawn_hive_visual, AppState, Bee, BeeKind, CorePlugin, HiveMap};

use bevy::{
    prelude::*,
    render::mesh::shape::Quad,
    sprite::{MaterialMesh2dBundle, Mesh2d, Mesh2dHandle},
};
use rand::Rng;

pub mod core;
pub mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CorePlugin);

    if utils::is_local_build() {
        app.add_systems(Startup, setup);
        app.add_systems(Update, spawn_bees);
    } else {
    }

    app.run();
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

    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(1.0 / 0.1)),
        ..default()
    });

    spawn_hive_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );

    clear_color.0 = Color::WHITE;
}

fn spawn_bees(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut done: Local<bool>,
    map: Res<HiveMap>,
    time: Res<Time>,
) {
    if *done || !map.ready {
        return;
    }

    for i in 0..16 {
        let x = rand::thread_rng().gen_range(-200.0..200.0);
        let y = rand::thread_rng().gen_range(-200.0..200.0);
        let z = rand::thread_rng().gen_range(-1.0..1.0);

        if map.get_obstruction(Vec2::new(x, y)) > 0.3 {
            continue;
        }

        commands.spawn((
            Mesh2dHandle(meshes.add(Quad::new(Vec2::new(24.0, 24.0)).into())),
            Bee {
                kind: BeeKind::Defender,
                target: Vec2::ZERO,
            },
            TransformBundle::from_transform(
                Transform::from_xyz(x, y, z).with_scale(Vec3::new(-1.0, 1.0, 1.0)),
            ),
            VisibilityBundle::default(),
        ));
    }

    *done = true;
}
