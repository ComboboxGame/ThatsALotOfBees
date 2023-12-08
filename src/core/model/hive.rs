use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};

use crate::core::{
    AppState, MouseState
};

pub const HIVE_WORLD_SIZE: f32 = 320.0;
pub const HIVE_IMAGE_SIZE: usize = 160;

pub const BUILDINGS_NUM: usize = 11;

pub const BUILDING_POSITIONS: [(u32, u32); BUILDINGS_NUM] = [
    (34, 68),
    (42, 102),
    (54, 36),
    (65, 66),
    (64, 127),
    (77, 96),
    (88, 32),
    (97, 69),
    (100, 122),
    (119, 48),
    (118, 95),
];

pub fn get_building_position(index: usize) -> Vec2 {
    let x =
        ((BUILDING_POSITIONS[index].0 + 1) as f32 / HIVE_IMAGE_SIZE as f32 - 0.5) * HIVE_WORLD_SIZE;
    let y =
        ((BUILDING_POSITIONS[index].1 + 1) as f32 / HIVE_IMAGE_SIZE as f32 - 0.5) * HIVE_WORLD_SIZE;
    Vec2::new(x, -y)
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingKind {
    #[default]
    None,
    Nexus,
    Storage,
    WaxReactor,
    Armory,
    Workshop,
    BuilderAcademy,
}

impl ToString for BuildingKind {
    fn to_string(&self) -> String {
        match self {
            BuildingKind::None => String::from("Empty lot"),
            BuildingKind::Nexus => String::from("Nexus"),
            BuildingKind::Storage => String::from("Storage"),
            BuildingKind::WaxReactor => String::from("Wax reactor"),
            BuildingKind::Armory => String::from("Armory"),
            BuildingKind::Workshop => String::from("Workshop"),
            BuildingKind::BuilderAcademy => String::from("Builder academy"),
        }
    }
}

pub fn get_building_image_name(kind: BuildingKind) -> &'static str {
    match kind {
        BuildingKind::None => "images/None.png",
        BuildingKind::Nexus => "images/Nexus.png",
        BuildingKind::Storage => "images/Nexus.png",
        BuildingKind::WaxReactor => "images/Nexus.png",
        BuildingKind::Armory => "images/Nexus.png",
        BuildingKind::Workshop => "images/Nexus.png",
        BuildingKind::BuilderAcademy => "images/Nexus.png",
    }
}

#[derive(Resource)]
pub struct HiveBuildings {
    pub buildings: [BuildingKind; BUILDINGS_NUM],
}

impl Default for HiveBuildings {
    fn default() -> Self {
        let mut buildings: [BuildingKind; BUILDINGS_NUM] = Default::default();
        buildings[8] = BuildingKind::Nexus;
        Self { buildings }
    }
}

#[derive(Component)]
pub struct Building {
    pub kind: BuildingKind,
    pub index: usize,
    pub time_bank: f32,
    pub queen_spawned: bool,
}

pub fn update_buildings_system(
    mut commands: Commands,
    buildings: Res<HiveBuildings>,
    buildings_query: Query<(Entity, &Building, &Handle<ColorMaterial>)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    state: Res<State<AppState>>,
) {
    let mut exists = [false; BUILDINGS_NUM];
    for (e, building, _) in buildings_query.iter() {
        if building.kind != buildings.buildings[building.index] || *state.get() != AppState::InGame
        {
            commands.entity(e).despawn();
        } else {
            exists[building.index] = true;
        }
    }

    for index in 0..BUILDINGS_NUM {
        if exists[index] || *state.get() != AppState::InGame {
            continue;
        }

        commands.spawn((
            Building {
                kind: buildings.buildings[index],
                index,
                time_bank: 0.0,
                queen_spawned: false,
            },
            TransformBundle::from_transform(Transform::from_translation(
                get_building_position(index).extend(-5.0),
            )),
            VisibilityBundle::default(),
            materials.add(ColorMaterial::from(
                asset_server.load(get_building_image_name(buildings.buildings[index])),
            )),
            Mesh2dHandle(meshes.add(Quad::new(Vec2::new(64.0, 64.0)).into())),
        ));
    }
}
