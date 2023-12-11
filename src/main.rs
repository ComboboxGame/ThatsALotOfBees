use crate::core::{AppState, CorePlugin};

use bevy::{
    ecs::schedule::IntoSystemConfigs,
    prelude::*,
    render::texture::{ImageFilterMode, ImageSamplerDescriptor},
    sprite::Mesh2dHandle, asset::AssetLoader,
};
use levels::{LevelsPlugin, Scenario0};
use utils::FpsPlugin;

pub mod core;
pub mod levels;
pub mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(ImagePlugin {
        default_sampler: ImageSamplerDescriptor {
            mag_filter: ImageFilterMode::Nearest,
            min_filter: ImageFilterMode::Nearest,
            mipmap_filter: ImageFilterMode::Nearest,
            ..Default::default()
        },
    }))
    .add_plugins(CorePlugin)
    .add_plugins(LevelsPlugin);

    app.add_systems(Startup, (camera_setup, preload_assets));

    app.add_systems(PostUpdate, cleanup.run_if(state_changed::<AppState>()));

    //if utils::is_local_build()
    {
        //app.add_plugins(FpsPlugin);
        //app.add_systems(Startup, go_to_game_immediately);
        //app.add_systems(Update, state_debug_system);
    }
    
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(1.0 / 0.1)),
        ..default()
    });
}

pub fn cleanup(
    everything: Query<Entity, Or<(With<Mesh2dHandle>, With<Style>)>>,
    scenarios: Query<Entity, With<Scenario0>>,
    other: Query<Entity, (Without<Mesh2dHandle>, Without<Style>)>,
    state: Res<State<AppState>>,
    mut commands: Commands,
) {
    println!("Cleanup");
    for e in everything.iter() {
        commands.entity(e).despawn();
    }
    if *state.get() == AppState::MainMenu {
        for e in scenarios.iter() {
            commands.entity(e).despawn();
        }
    }
    for o in other.iter() {
        println!("{:?}", o);
    }
}

fn go_to_game_immediately(mut next_state: ResMut<NextState<AppState>>, mut commands: Commands) {
    println!("Go to game");
    next_state.set(AppState::InGame);
}

fn state_debug_system(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Q) {
        match *state.get() {
            AppState::InGame => {
                next_state.set(AppState::MainMenu);
            }
            AppState::MainMenu => {
                next_state.set(AppState::InGame);
            }
        }
    }
}

fn preload_assets(
    asset_server: Res<AssetServer>,
    mut bank: Local<Vec<Handle<Image>>>,
) {
    let names = vec![
        "images/MainBackground.png",
        "images/Tree.png",
        "images/Clouds.png",
        "images/Hills1.png",
        "images/Hills2.png",
        "images/DisabledButton.png",
        "images/EnabledButton.png",
        "images/DisabledHoveredButton.png",
        "images/HoveredButton.png",
        "images/NexusMenu2.png",
        "images/MagicWaxReactorMenu.png",
        "images/WaxReactorMenu.png",
        "images/ArmoryMenu.png",
        "images/WorkshopMenu.png",
        "images/ResourcesMenu.png",
        "images/CounterMenu.png",
        "images/BuildMenu.png",
        "images/Hive.png",
    ];

    for name in names {
        bank.push(asset_server.load(name));
    }
}