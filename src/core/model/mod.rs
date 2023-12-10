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
use bevy::{prelude::*, render::mesh::shape::Quad, sprite::{Material2dPlugin, Mesh2dHandle}, utils::HashMap, ui::FocusPolicy};
pub use buildings::*;
pub use currency::*;
pub use enemy::*;
pub use living_creature::*;
pub use material::*;
pub use physcis::*;
use rand::{thread_rng, Rng};

use crate::{core::{spawn_hive_visual, get_view_rect}, levels::{NextWave, Scenario0}};

use self::behaviours::BehaviourPlugin;

use super::{AppState, FONT_HANDLE, RelativePixelFont, RelativePixelSized};

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
        app.init_resource::<GameInfo>();

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

        app.add_systems(Update, game_end_system.run_if(in_state(AppState::InGame)));
        app.add_systems(Update, buttons);

        app.add_plugins(BehaviourPlugin);
    }
}

pub fn create_meshes(mut meshes: ResMut<Assets<Mesh>>) {
    meshes.insert(BEE_MESH, Quad::new(Vec2::splat(24.0)).into());
    meshes.insert(WASP_MESH, Quad::new(Vec2::splat(24.0)).into());
    meshes.insert(BIRB_MESH, Quad::new(Vec2::splat(24.0)).into());
}

#[derive(Component)]
pub struct Play;

fn entered_main_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::ZERO,
                top: Val::ZERO,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        //UiImage::new(asset_server.load("images/MainBackground.png")),
    )).with_children(|builder| {
        builder.spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
            Interaction::None,
            RelativePixelSized {
                width: 88 * 2,
                height: 32 * 2
            },
            UiImage::new(asset_server.load("images/Play.png")),
            Play,
        ));

        builder.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(2.0),
                    bottom: Val::Percent(0.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            },
            RelativePixelSized {
                width: 70,
                height: 32
            },
            UiImage::new(asset_server.load("images/Authors.png")),
            Play,
        ));
    });

    commands.spawn(
        BeeBundle {
            ..BeeBundle::from((BeeType::Defender(1), Vec2::new(thread_rng().gen_range(-100.0..100.0), thread_rng().gen_range(-100.0..100.0))))
        }
    );
    commands.spawn(
        BeeBundle {
            ..BeeBundle::from((BeeType::Regular, Vec2::new(thread_rng().gen_range(-100.0..100.0), thread_rng().gen_range(-100.0..100.0))))
        }
    );
    commands.spawn(
        BeeBundle {
            ..BeeBundle::from((BeeType::Regular, Vec2::new(thread_rng().gen_range(-100.0..100.0), thread_rng().gen_range(-100.0..100.0))))
        }
    );

    commands.spawn((
        materials.add(ColorMaterial::from(asset_server.load("images/MainBackground.png"))),
        Mesh2dHandle(meshes.add(Quad::new(Vec2::new(2560.0 / 4.0, 1440.0 / 4.0)).into())),
        TransformBundle::from_transform(Transform::from_xyz(0., 0., -10.)),
        VisibilityBundle::default(),
    ));
}

#[derive(Component)]
pub struct BackToMenu;

#[derive(Component)]
pub struct Pause;

fn game_end_system(
    mut commands: Commands,
    game_end: Res<GameInfo>,
    mut menu_spawned: Local<bool>,
    back: Query<&Interaction, With<BackToMenu>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !game_end.end {
        return;
    }

    if !*menu_spawned {
        commands.spawn(NodeBundle { 
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::ZERO,
                top: Val::ZERO,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            focus_policy: FocusPolicy::Block,
            z_index: ZIndex::Global(200),
            background_color: BackgroundColor(Color::rgba_u8(40, 40, 40, 238)),
            ..default()
        }).with_children(|builder| {
            builder.spawn(
                (TextBundle {
                    text: Text::from_section("Queen died...", TextStyle { font: FONT_HANDLE, font_size: 20.0, color: Color::rgb(0.05, 0.02, 0.02) }),
                    style: Style {
                        margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Auto),
                        ..default()
                    },
                    ..default()
                },
                RelativePixelFont {
                    size: 60,
                },)
            );
            builder.spawn(
                (TextBundle {
                    text: Text::from_section("Back to menu", TextStyle { font: FONT_HANDLE, font_size: 20.0, color: Color::rgb(0.9, 0.9, 0.9) }),
                    style: Style {
                        margin: UiRect::new(Val::Auto, Val::Auto, Val::ZERO, Val::Percent(9.0)),
                        ..default()
                    },
                    ..default()
                },
                Interaction::None,
                RelativePixelFont {
                    size: 12,
                }, BackToMenu)
            );
        });

        *menu_spawned = true;
    }

    for b in back.iter() {
        if *b == Interaction::Pressed {
            next_state.set(AppState::MainMenu);
        }
    }
}

fn enetered_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: ResMut<AssetServer>,
    mut hive_buildings: ResMut<HiveBuildings>,
    mut currency: ResMut<CurrencyStorage>,
    mut game_end: ResMut<GameInfo>,
    mut cameras: Query<
        (&Camera, &mut Transform)
    >,
) {
    *hive_buildings = HiveBuildings::default();
    *currency = CurrencyStorage::default();
    *game_end = GameInfo::default();

    commands.spawn(BeeBundle::from((BeeType::Queen, get_building_position(8))));

    spawn_hive_visual(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );

    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(2.0),
                top: Val::Percent(2.0),
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        },
        Interaction::None,
        UiImage::new(asset_server.load("images/Pause.png")),
        RelativePixelSized {
            width: 32,
            height: 32,
        },
        Pause,
    ));

    commands.spawn((
        TextBundle {
            text: Text::from_section("", TextStyle { font: FONT_HANDLE, font_size: 10.0, color: Color::BLACK }),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(2.0),
                top: Val::Percent(2.0),
                ..default()
            },
            ..default()
        },
        RelativePixelFont {
            size: 20,
        },
        NextWave,
    ));

    commands.spawn(Scenario0::default());
}

fn buttons(
    mut button: Query<(&Interaction, &mut BackgroundColor), (With<Pause>, Without<Play>, Changed<Interaction>)>,
    mut button_play: Query<(&Interaction, &mut BackgroundColor), (With<Play>, Without<Pause>, Changed<Interaction>)>,
    mut game: ResMut<GameInfo>,
    mut state: ResMut<NextState<AppState>>,
    state_cur: ResMut<State<AppState>>,
    mut cameras: Query<
        (&Camera, &mut Transform)
    >,
)
{
    for (interaction, mut color) in button.iter_mut() {
        if *interaction != Interaction::None {
            *color = BackgroundColor(Color::rgb(0.8, 0.8, 0.8));
        } else {
            *color = BackgroundColor(Color::WHITE);
        }
        if *interaction == Interaction::Pressed {
            game.paused = !game.paused;
        }
    }
    for (interaction, mut color) in button_play.iter_mut() {
        if *interaction != Interaction::None {
            *color = BackgroundColor(Color::rgb(0.9, 0.9, 0.9));
        } else {
            *color = BackgroundColor(Color::WHITE);
        }
        if *interaction == Interaction::Pressed {
            state.set(AppState::InGame);
        }
    }

    if *state_cur.get() == AppState::MainMenu {
        for (camera, mut transform) in cameras.iter_mut() {
            transform.translation = Vec3::ZERO;
    
            let view_rect = get_view_rect(camera, &transform);
            let view_size = view_rect.max - view_rect.min;
            transform.scale *= 1440.0 * 0.25 / view_size.y;
            let view_rect = get_view_rect(camera, &transform);
            let view_size = view_rect.max - view_rect.min;
            transform.scale *= 2560.0 * 0.25 / view_size.x;
        }
    }
}