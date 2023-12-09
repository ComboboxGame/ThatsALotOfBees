use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    core::{AppState, BeeBundle},
    utils::FlatProvider,
};

use super::{
    currency, BeeType, BuildingMaterial, CurrencyGainPerMinute, CurrencyStorage, CurrencyValues,
    LivingCreature, RigidBody, UniversalBehaviour, UniversalMaterial, MAX_DEFENDER_LEVEL,
    MAX_WORKER_LEVEL,
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
    MagicWaxReactor,
}

impl BuildingKind {
    pub fn get_menu_image(&self) -> &'static str {
        match self {
            BuildingKind::None => "images/None.png",
            BuildingKind::Nexus => "images/NexusMenu.png",
            BuildingKind::Storage => "images/None.png",
            BuildingKind::WaxReactor => "images/WaxReactorMenu.png",
            BuildingKind::Armory => "images/ArmoryMenu.png",
            BuildingKind::Workshop => "images/WorkshopMenu.png",
            BuildingKind::MagicWaxReactor => "images/MagicWaxReactorMenu.png",
        }
    }
    pub fn get_menu_size(&self) -> (u32, u32) {
        match self {
            BuildingKind::Workshop | BuildingKind::Armory => (114, 28 * 3 + 1),
            _ => (114, 28 * 2 + 1),
        }
    }
}

impl ToString for BuildingKind {
    fn to_string(&self) -> String {
        match self {
            BuildingKind::None => String::from("Empty lot"),
            BuildingKind::Nexus => String::from("Birther"),
            BuildingKind::Storage => String::from("Honey storage"),
            BuildingKind::WaxReactor => String::from("Wax reactor"),
            BuildingKind::Armory => String::from("Defender bee school"),
            BuildingKind::Workshop => String::from("Worker bee school"),
            BuildingKind::MagicWaxReactor => String::from("Magic wax reactor"),
        }
    }
}

pub fn get_building_image_name(kind: BuildingKind) -> &'static str {
    match kind {
        BuildingKind::None => "images/None.png",
        BuildingKind::Nexus => "images/Nexus.png",
        BuildingKind::Storage => "images/Storage.png",
        BuildingKind::WaxReactor => "images/WaxReactor.png",
        BuildingKind::Armory => "images/Armory.png",
        BuildingKind::Workshop => "images/Workshop.png",
        BuildingKind::MagicWaxReactor => "images/MagicWaxReactor.png",
    }
}

#[derive(Resource)]
pub struct HiveBuildings {
    pub buildings: [BuildingKind; BUILDINGS_NUM],

    pub build_order: Option<(BuildingKind, usize)>,
    pub upgrade_order: Option<usize>,
    pub destroy_order: Option<usize>,

    pub defender_lvl: u32,
    pub worker_lvl: u32,

    pub any_order_done: bool,
    pub any_upgrade_done: bool,

    pub storages: u32,
}

impl Default for HiveBuildings {
    fn default() -> Self {
        let mut buildings: [BuildingKind; BUILDINGS_NUM] = Default::default();
        buildings[8] = BuildingKind::Nexus;
        /*buildings[6] = BuildingKind::Armory;
        buildings[4] = BuildingKind::Workshop;
        buildings[2] = BuildingKind::WaxReactor;
        buildings[9] = BuildingKind::WaxReactor;
        buildings[3] = BuildingKind::Storage;
        buildings[7] = BuildingKind::Storage;
        buildings[0] = BuildingKind::MagicWaxReactor;*/
        Self {
            buildings,
            build_order: None,
            upgrade_order: None,
            destroy_order: None,
            any_order_done: false,
            any_upgrade_done: false,
            defender_lvl: 0,
            storages: 0,
            worker_lvl: 0,
        }
    }
}

impl HiveBuildings {
    pub fn get_build_cost(&self, kind: BuildingKind) -> CurrencyValues {
        match kind {
            BuildingKind::None => CurrencyValues::default(),
            BuildingKind::Nexus => CurrencyValues::default(),
            BuildingKind::Workshop => [8, 2, 0],
            BuildingKind::Armory => [8, 8, 0],
            BuildingKind::Storage => [50, 10, 0],
            BuildingKind::WaxReactor => [99, 19, 0],
            BuildingKind::MagicWaxReactor => [0, 99, 19],
        }
    }

    pub fn get_current_defender(&self) -> BeeType {
        BeeType::Defender(self.defender_lvl)
    }

    pub fn get_current_worker(&self) -> BeeType {
        BeeType::Worker(self.worker_lvl)
    }

    pub fn get_next_defender(&self) -> BeeType {
        BeeType::Defender((self.defender_lvl + 1).min(MAX_DEFENDER_LEVEL - 1))
    }

    pub fn get_next_worker(&self) -> BeeType {
        BeeType::Worker((self.worker_lvl + 1).min(MAX_WORKER_LEVEL - 1))
    }

    pub fn get_order_cost(&self, kind: BuildingKind) -> CurrencyValues {
        match kind {
            BuildingKind::None => CurrencyValues::default(),
            BuildingKind::Nexus => [1, 0, 0],
            BuildingKind::Storage => CurrencyValues::default(),
            BuildingKind::Armory => [
                0,
                4 * (self.defender_lvl + 1) as u64,
                1 * (self.defender_lvl + 1) as u64,
            ],
            BuildingKind::Workshop => [
                4 * (self.worker_lvl + 1) as u64,
                1 * (self.worker_lvl + 1) as u64,
                0,
            ],
            BuildingKind::WaxReactor => [40, 4, 0],
            BuildingKind::MagicWaxReactor => [160, 0, 4],
        }
    }

    pub fn get_upgrade_cost(&self, kind: BuildingKind) -> CurrencyValues {
        match kind {
            BuildingKind::None => CurrencyValues::default(),
            BuildingKind::Nexus => [0, 0, 5],
            BuildingKind::Storage => CurrencyValues::default(),
            BuildingKind::Armory => [
                0,
                24 * (self.defender_lvl + 1) as u64,
                8 * (self.defender_lvl + 1) as u64,
            ],
            BuildingKind::Workshop => [
                0,
                16 * (self.worker_lvl + 1) as u64,
                8 * (self.worker_lvl + 1) as u64,
            ],
            BuildingKind::WaxReactor => CurrencyValues::default(),
            BuildingKind::MagicWaxReactor => CurrencyValues::default(),
        }
    }

    pub fn get_current_level(&self, bee: BeeType) -> BeeType {
        match bee {
            BeeType::Baby => BeeType::Baby,
            BeeType::Regular => BeeType::Regular,
            BeeType::Worker(_) => BeeType::Worker(self.worker_lvl),
            BeeType::Defender(_) => BeeType::Defender(self.defender_lvl),
            BeeType::Queen => BeeType::Queen,
        }
    }

    pub fn get_max_honey(&self) -> u64 {
        50 + (self.storages * 50) as u64
    }

    pub fn get_max_storages(&self) -> u32 {
        4
    }

    pub fn get_upgrade_name(&self, kind: BuildingKind) -> &'static str {
        match kind {
            BuildingKind::Nexus => "Random shields",
            BuildingKind::Armory => ["Research Rambo bee", "Research Cybernetic defender", ""][self.defender_lvl as usize],
            BuildingKind::Workshop => ["Research Crazy worker", "Research Robo worker", ""][self.worker_lvl as usize],
            _ => "",
        }
    }

    pub fn get_order_name(&self, kind: BuildingKind) -> &'static str {
        match kind {
            BuildingKind::Nexus => "Baby bee",
            BuildingKind::Armory => ["Defender", "Rambo bee", "Cybernetic defender"][self.defender_lvl as usize],
            BuildingKind::Workshop => ["Worker", "Crazy worker", "Roboworker"][self.worker_lvl as usize],
            BuildingKind::WaxReactor => "Wax synthesis",
            BuildingKind::MagicWaxReactor => "Magic wax synthesis",
            _ => "",
        }
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
}

impl Building {
    pub fn order(&mut self) {
        self.orders_stashed_count += 1
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
                material.progress.x =
                    1.0 - (building.order_time_remaining / building.order_time).max(0.0);
            } else {
                material.progress = Vec4::splat(0.0);
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
                order_time: if buildings.buildings[index] == BuildingKind::Nexus {
                    3.0
                } else {
                    5.0
                },
                order_time_remaining: 0.0,
                orders_count: 0,
                orders_stashed_count: 0,
            },
            TransformBundle::from_transform(Transform::from_translation(
                get_building_position(index).extend(-5.0),
            )),
            VisibilityBundle::default(),
            materials.add(BuildingMaterial {
                progress: Vec4::ZERO,
                texture: Some(texture),
                background: Some(background),
                selected: Some(selected),
                hovered: Some(hovered),
                state: UVec4::ZERO,
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
    mut bees: Query<(
        &mut BeeType,
        &mut UniversalBehaviour,
        &mut LivingCreature,
        &mut RigidBody,
        &mut CurrencyGainPerMinute,
        &Handle<UniversalMaterial>,
    )>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    time: Res<Time>,
    mut rng: Local<Option<StdRng>>,
    mut currency: ResMut<CurrencyStorage>,
    mut hive_buildings: ResMut<HiveBuildings>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    if let Some(build_order) = hive_buildings.build_order.take() {
        let cost = hive_buildings.get_build_cost(build_order.0);
        if currency.check_can_spend(&cost)
            && (build_order.0 != BuildingKind::Storage
                || hive_buildings.storages < hive_buildings.get_max_storages())
        {
            currency.spend(&cost);
            hive_buildings.buildings[build_order.1] = build_order.0;
            hive_buildings.any_order_done = true;

            if build_order.0 == BuildingKind::Storage {
                let storages = hive_buildings
                    .buildings
                    .iter()
                    .filter(|f| **f == BuildingKind::Storage)
                    .count() as u32;
                hive_buildings.storages = storages;
                currency.max_stored[0] = hive_buildings.get_max_honey();
            }
        }
    }

    if let Some(upgrade_order) = hive_buildings.upgrade_order.take() {
        let kind = hive_buildings.buildings[upgrade_order];
        let cost = hive_buildings.get_upgrade_cost(kind);
        if currency.check_can_spend(&cost) {
            if kind == BuildingKind::Workshop {
                hive_buildings.worker_lvl = (hive_buildings.worker_lvl + 1).min(MAX_WORKER_LEVEL - 1);
                hive_buildings.any_upgrade_done = true;
                currency.spend(&cost);
            }
            if kind == BuildingKind::Armory {
                hive_buildings.defender_lvl = (hive_buildings.defender_lvl + 1).min(MAX_DEFENDER_LEVEL - 1);
                hive_buildings.any_upgrade_done = true;
                currency.spend(&cost);
            }
            if kind == BuildingKind::Nexus {
                // todo: apply shields
                hive_buildings.any_upgrade_done = true;
                currency.spend(&cost);
            }
        }
        /*let cost = hive_buildings.get_build_cost(build_order.0);
        if currency.check_can_spend(&cost) {
            currency.spend(&cost);
            hive_buildings.buildings[build_order.1] = build_order.0;
            any_order_done = true;
        }*/
    }

    if let Some(destroy_order) = hive_buildings.destroy_order.take() {
        if hive_buildings.buildings[destroy_order] != BuildingKind::Nexus {
            hive_buildings.buildings[destroy_order] = BuildingKind::None;
            hive_buildings.any_order_done = true;
        }
    }

    if *bee_mesh == Handle::default() {
        *bee_mesh = meshes.add(Quad::new(Vec2::new(24.0, 24.0)).into());
    }

    for (mut building, transform) in buildings.iter_mut() {
        while building.orders_stashed_count > 0 {
            building.orders_stashed_count -= 1;

            let mut cost = hive_buildings.get_order_cost(building.kind);

            if building.kind == BuildingKind::WaxReactor {
                cost[1] = 0;
            }
            if building.kind == BuildingKind::MagicWaxReactor {
                cost[2] = 0;
            }

            if !currency.check_can_spend(&cost) {
                break;
            }

            currency.spend(&cost);

            if building.orders_count == 0 {
                building.order_time_remaining = building.order_time;
            }
            building.orders_count += 1;
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
            BuildingKind::None => {}
            BuildingKind::Nexus => {
                // spawn baby
                let (mut x, mut y) = (100.0, 100.0);
                while x * x + y * y > 20.0 * 20.0 {
                    (x, y) = (rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
                }
                commands.spawn(BeeBundle::from((
                    BeeType::Baby,
                    Vec2::new(x, y) + transform.flat(),
                )));
                success = true;
            }
            BuildingKind::Storage => {},
            BuildingKind::WaxReactor => {
                let cost = hive_buildings.get_order_cost(BuildingKind::WaxReactor);
                currency.stored[1] += cost[1];
                // Wax reactor
                success = true;
            }
            BuildingKind::MagicWaxReactor => {
                let cost = hive_buildings.get_order_cost(BuildingKind::MagicWaxReactor);
                currency.stored[2] += cost[2];
                // Wax reactor
                success = true;
            }
            BuildingKind::Armory => {
                for (mut bee, mut behaviour, mut creature, mut rb, mut gain, material) in
                    bees.iter_mut()
                {
                    if *bee == BeeType::Regular && !creature.is_dead() {
                        let b = BeeType::Defender(hive_buildings.defender_lvl);
                        *bee = b;
                        *behaviour = UniversalBehaviour::from(b);
                        *creature = LivingCreature::from(b);
                        *rb = RigidBody::from(b);
                        *gain = CurrencyGainPerMinute::from(b);
                        success = true;
                        if let Some(material) = materials.get_mut(material) {
                            material.props.upgrade_time = time.elapsed_seconds();
                        }
                        break;
                    }
                }
            }
            BuildingKind::Workshop => {
                for (mut bee, mut behaviour, mut creature, mut rb, mut gain, material) in
                    bees.iter_mut()
                {
                    if *bee == BeeType::Regular && !creature.is_dead() {
                        let b = BeeType::Worker(hive_buildings.worker_lvl);
                        *bee = b;
                        *behaviour = UniversalBehaviour::from(b);
                        *creature = LivingCreature::from(b);
                        *rb = RigidBody::from(b);
                        *gain = CurrencyGainPerMinute::from(b);
                        if let Some(material) = materials.get_mut(material) {
                            material.props.upgrade_time = time.elapsed_seconds();
                        }
                        success = true;
                        break;
                    }
                }
            }
        }

        if success {
            building.orders_count -= 1;
            if building.orders_count > 0 {
                building.order_time_remaining = building.order_time;
            }
        }
    }

    if hive_buildings.any_upgrade_done {
        // just upgraded, check if bees need upgrading

        for (mut bee, mut behaviour, mut creature, mut rb, mut gain, material) in bees.iter_mut() {
            let expected_bee = hive_buildings.get_current_level(*bee);

            if *bee != expected_bee && !creature.is_dead() {
                let b = expected_bee;
                *bee = b;
                *behaviour = UniversalBehaviour::from(b);
                *creature = LivingCreature::from(b);
                *rb = RigidBody::from(b);
                *gain = CurrencyGainPerMinute::from(b);
                if let Some(material) = materials.get_mut(material) {
                    material.props.upgrade_time = time.elapsed_seconds();
                }
            }
        }
    }
}
