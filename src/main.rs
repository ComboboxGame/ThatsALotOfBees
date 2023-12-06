use core::{spawn_hive_visual, AppState, CorePlugin};

use bevy::prelude::*;

pub mod core;
pub mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CorePlugin);

    app.add_systems(Startup, camera_setup);

    if utils::is_local_build() {
        app.add_systems(Startup, setup);
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
