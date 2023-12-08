use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};
use rand::{rngs::StdRng, SeedableRng, Rng};

use crate::{core::{AppState, BeeBundle}, utils::FlatProvider};

use super::{BeeType, UniversalBehaviour, LivingCreature, RigidBody, BuildingMaterial, CurrencyValues, CurrencyStorage, currency, CurrencyGainPerMinute, UniversalMaterial};

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

    pub orders_stashed_count: u32,

    // todo??? remove this field
    pub queen_spawned: bool,
}

impl Building {
    pub fn order(&mut self) {
        self.orders_stashed_count += 1
    }

    pub fn get_order_cost(&self) -> CurrencyValues {
        match self.kind {
            BuildingKind::None => CurrencyValues::default(),
            BuildingKind::Nexus => [1, 0, 0],
            BuildingKind::Storage => CurrencyValues::default(),
            BuildingKind::WaxReactor => CurrencyValues::default(),
            BuildingKind::Armory => [4, 1, 1],
            BuildingKind::Workshop => [4, 0, 1],
            BuildingKind::BuilderAcademy => CurrencyValues::default(),
        }
    }
}

pub fn update_buildings_system(
    mut commands: Commands,
    buildings: Res<HiveBuildings>,
    buildings_query: Query<(Entity, &Building, &Handle<BuildingMaterial>)>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<BuildingMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    state: Res<State<AppState>>,
) {
    let mut exists = [false; BUILDINGS_NUM];
    for (e, building, material) in buildings_query.iter() {
        if building.kind != buildings.buildings[building.index] || *state.get() != AppState::InGame
        {
            commands.entity(e).despawn();
        } else {
            exists[building.index] = true;
        }

        if let Some(material) = materials.get_mut(material) {
            if building.order_time_remaining > 0.0 || building.orders_count > 0 {
                material.progress = 1.0 - (building.order_time_remaining / building.order_time).max(0.0);
            } else {
                material.progress = 0.0;
            }
        }
    }

    for index in 0..BUILDINGS_NUM {
        if exists[index] || *state.get() != AppState::InGame {
            continue;
        }

        let texture = asset_server.load(get_building_image_name(buildings.buildings[index]));
        let background = asset_server.load("images/BuildingProgress.png");
        let selected = asset_server.load("images/BuildingSelected.png");
        let hovered = asset_server.load("images/BuildingHovered.png");

        commands.spawn((
            Building {
                kind: buildings.buildings[index],
                index,
                queen_spawned: false,
                order_time: 2.0,
                order_time_remaining: 0.0,
                orders_count: 0,
                orders_stashed_count: 0,
            },
            TransformBundle::from_transform(Transform::from_translation(
                get_building_position(index).extend(-5.0),
            )),
            VisibilityBundle::default(),
            materials.add(BuildingMaterial {
                progress: 0.0,
                texture: Some(texture),
                background: Some(background),
                selected: Some(selected),
                hovered: Some(hovered),
                state: 0,
            }),
            Mesh2dHandle(meshes.add(Quad::new(Vec2::new(64.0, 64.0)).into())),
        ));
    }
}

pub fn buildings_system(
    mut commands: Commands,
    mut buildings: Query<(&mut Building, &Transform)>,
    mut bee_mesh: Local<Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut bees: Query<(&mut BeeType, &mut UniversalBehaviour, &mut LivingCreature, &mut RigidBody, &mut CurrencyGainPerMinute, &Handle<UniversalMaterial>)>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    time: Res<Time>,
    mut rng: Local<Option<StdRng>>,
    mut currency: ResMut<CurrencyStorage>,
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

        while building.orders_stashed_count > 0 {
            building.orders_stashed_count -= 1;

            if !currency.check_can_spend(&building.get_order_cost()) {
                break;
            }

            currency.spend(&building.get_order_cost());

            if building.orders_count == 0 {
                building.order_time_remaining = building.order_time;
            }
            building.orders_count += 1;
        }

        if building.kind == BuildingKind::Workshop {
            println!("Orders: {}", building.orders_count);
        }

        if building.orders_count == 0 {
            continue;
        }

        building.order_time_remaining -= time.delta_seconds();

        if building.order_time_remaining > 0.0 {
            continue;
        }

        let mut success = false;

        match building.kind {
            BuildingKind::None => {},
            BuildingKind::Nexus => {
                // spawn baby
                let (mut x, mut y) = (100.0, 100.0);
                while x * x + y * y > 20.0 * 20.0 {
                    (x, y) = (rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
                }
                commands.spawn(BeeBundle::from((BeeType::Baby, Vec2::new(x, y) + transform.flat())));
                success = true;
            },
            BuildingKind::Storage => todo!(),
            BuildingKind::WaxReactor => todo!(),
            BuildingKind::Armory => {
                for (mut bee, mut behaviour, mut creature, mut rb, mut gain, material) in bees.iter_mut() {
                    if *bee == BeeType::Regular && !creature.is_dead() {
                        *bee = BeeType::Defender;
                        *behaviour = UniversalBehaviour::from(BeeType::Defender);
                        *creature = LivingCreature::from(BeeType::Defender);
                        *rb = RigidBody::from(BeeType::Defender);
                        *gain = CurrencyGainPerMinute::from(BeeType::Defender);
                        success = true;
                        if let Some(material) = materials.get_mut(material) {
                            material.props.upgrade_time = time.elapsed_seconds();
                        }
                        break;
                    }
                }
            },
            BuildingKind::Workshop => {
                for (mut bee, mut behaviour, mut creature, mut rb, mut gain, material) in bees.iter_mut() {
                    if *bee == BeeType::Regular && !creature.is_dead() {
                        *bee = BeeType::Worker;
                        *behaviour = UniversalBehaviour::from(BeeType::Worker);
                        *creature = LivingCreature::from(BeeType::Worker);
                        *rb = RigidBody::from(BeeType::Worker);
                        *gain = CurrencyGainPerMinute::from(BeeType::Worker);
                        if let Some(material) = materials.get_mut(material) {
                            material.props.upgrade_time = time.elapsed_seconds();
                        }
                        success = true;
                        break;
                    }
                }
            },
            BuildingKind::BuilderAcademy => {},
        }

        if success {
            building.orders_count -= 1;
            if building.orders_count > 0 {
                building.order_time_remaining = building.order_time;
            }
        }

        
    }
}
