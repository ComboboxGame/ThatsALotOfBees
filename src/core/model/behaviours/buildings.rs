use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    core::{
        model::behaviours::universal_behaviour::UniversalBehaviour, BeeType, Building,
        BuildingKind, Faction, LivingCreature, MoveToNavigationTargetBehaviour, NavigationResult,
        NavigationTarget, RigidBody, SmartOrientation,
    },
    utils::FlatProvider,
};

#[derive(Component, Default)]
pub struct Nexus {
    pub time_bank: f32,
    pub queen_spawned: bool,
}

pub fn nexus_system(
    mut commands: Commands,
    mut nexuses: Query<(&mut Building, &Transform)>,
    bees: Query<&BeeType>,
    mut bee_mesh: Local<Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
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

    for (mut nexus, transform) in nexuses.iter_mut() {
        if nexus.kind != BuildingKind::Nexus {
            continue;
        }
        nexus.time_bank += time.delta_seconds();

        let babies = bees.iter().filter(|b| **b == BeeType::Baby).count();
        const MAX_BABIES: usize = 300;

        let cooldown = 1.0 + (babies as f32) / 4.0;

        if babies >= MAX_BABIES {
            nexus.time_bank = 0.0;
        }

        if nexus.time_bank > cooldown {
            nexus.time_bank -= cooldown;
            // spawn baby
            let (mut x, mut y) = (100.0, 100.0);
            while x * x + y * y > 20.0 * 20.0 {
                (x, y) = (rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
            }
            let z = rng.gen_range(0.0..1.0);

            commands.spawn((
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_translation(
                    transform.flat().extend(0.0) + Vec3::new(x, y, z),
                )),
                Mesh2dHandle(bee_mesh.clone()),
                BeeType::Baby,
                LivingCreature::from(BeeType::Baby),
                RigidBody::from(BeeType::Baby),
                UniversalBehaviour::from(BeeType::Baby),
                NavigationTarget::None,
                NavigationResult::default(),
                MoveToNavigationTargetBehaviour,
                SmartOrientation,
                Faction::Bees,
            ));
        }

        if !nexus.queen_spawned {
            nexus.queen_spawned = true;
            commands.spawn((
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_translation(
                    transform.flat().extend(1.0),
                )),
                Mesh2dHandle(bee_mesh.clone()),
                BeeType::Queen,
                LivingCreature::from(BeeType::Queen),
                RigidBody::from(BeeType::Queen),
                UniversalBehaviour::from(BeeType::Queen),
                NavigationTarget::None,
                NavigationResult::default(),
                MoveToNavigationTargetBehaviour,
                SmartOrientation,
                Faction::Bees,
            ));
        }
    }
}



/*
Baby price - 1 honey

start with 5 honey
5 honey / min - base rate
1 honey / min - regular bee production


X bees make X bees per minute


up to 50 bees - 1 honey per bee
up to 100 bees - 2 honey per bee
up to 150 bees - 3 honey per bee
up to 200 bees - 4 honey per bee
more - 8 honey per bee

Waves
1 - 
2 -
3 -
4 -
5 -
6 -
7 -
8 -
9 -
10 -


*/