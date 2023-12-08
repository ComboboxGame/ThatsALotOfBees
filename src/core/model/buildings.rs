use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};
use rand::{rngs::StdRng, SeedableRng, Rng};

use crate::{core::{AppState, BeeBundle}, utils::FlatProvider};

use super::{BeeType, UniversalBehaviour, LivingCreature, RigidBody};

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
        BuildingKind::Armory => "images/Armory.png",
        BuildingKind::Workshop => "images/Workshop.png",
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
        buildings[6] = BuildingKind::Armory;
        buildings[4] = BuildingKind::Workshop;
        Self { buildings }
    }
}

#[derive(Component)]
pub struct Building {

    pub kind: BuildingKind,
    pub index: usize,

    pub order_time: f32,
    pub order_time_remaining: f32,

    pub orders_count: u32,

    // todo??? remove this field
    pub queen_spawned: bool,
}

impl Building {
    pub fn order(&mut self) {
        if self.orders_count == 0 {
            self.order_time_remaining = self.order_time;
        }
        self.orders_count += 1;
    }
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
                queen_spawned: false,
                order_time: 2.0,
                order_time_remaining: 0.0,
                orders_count: 0,
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

pub fn buildings_system(
    mut commands: Commands,
    mut buildings: Query<(&mut Building, &Transform)>,
    mut bee_mesh: Local<Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut bees: Query<(&mut BeeType, &mut UniversalBehaviour, &mut LivingCreature, &mut RigidBody)>,
    time: Res<Time>,
    mut rng: Local<Option<StdRng>>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    if *bee_mesh == Handle::default() {
        *bee_mesh = meshes.add(Quad::new(Vec2::new(24.0, 24.0)).into());
    }

    for (mut building, transform) in buildings.iter_mut() {

        if building.kind == BuildingKind::Nexus && !building.queen_spawned {
            building.queen_spawned = true;
            commands.spawn(BeeBundle::from((BeeType::Queen, transform.flat())));
        }

        if building.orders_count == 0 {
            continue;
        }

        building.order_time_remaining -= time.delta_seconds();

        if building.order_time_remaining > 0.0 {
            continue;
        }

        building.orders_count -= 1;
        if building.orders_count > 0 {
            building.order_time_remaining += building.order_time;
        }
        
        match building.kind {
            BuildingKind::None => {},
            BuildingKind::Nexus => {
                // spawn baby
                let (mut x, mut y) = (100.0, 100.0);
                while x * x + y * y > 20.0 * 20.0 {
                    (x, y) = (rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
                }
                commands.spawn(BeeBundle::from((BeeType::Baby, Vec2::new(x, y) + transform.flat())));
            },
            BuildingKind::Storage => todo!(),
            BuildingKind::WaxReactor => todo!(),
            BuildingKind::Armory => {
                let mut spawned = false;
                for (mut bee, mut behaviour, mut creature, mut rb) in bees.iter_mut() {
                    if *bee == BeeType::Regular {
                        
                    }
                }
                if !spawned {
                    // Return order back, waiting when bee will appear
                    building.orders_count += 1;
                }
            },
            BuildingKind::Workshop => todo!(),
            BuildingKind::BuilderAcademy => todo!(),
        }

        
    }
}
