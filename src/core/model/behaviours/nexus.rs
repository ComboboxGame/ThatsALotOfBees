use bevy::{prelude::*, sprite::Mesh2dHandle, render::mesh::shape::Quad};
use rand::{rngs::StdRng, SeedableRng, Rng};

use crate::core::{Bee, BeeKind, NavigationTarget, MoveToNavigationTargetBehaviour, Velocity, VelocityOriented, MaxSpeed};

#[derive(Component, Default)]
pub struct Nexus {
    pub time_bank: f32,
    pub queen_spawned: bool,
}

pub fn nexus_system(
    mut commands: Commands,
    mut nexuses: Query<(&mut Nexus, &Transform)>,
    bees: Query<&Bee>,
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
        nexus.time_bank += time.delta_seconds();

        let babies = bees.iter().filter(|b| b.kind == BeeKind::Baby).count();
        const MAX_BABIES: usize = 30;

        let cooldown = 2.0 + (babies as f32) / 2.0;

        if babies >= MAX_BABIES {
            nexus.time_bank = 0.0;
        }

        if nexus.time_bank > cooldown {
            nexus.time_bank -= cooldown;
            // spawn baby
            let (mut x, mut y) = (100.0, 100.0);
            while x*x + y*y > 20.0 * 20.0 {
                (x, y) = (rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0));
            }
            let z = rng.gen_range(0.0..1.0);

            commands.spawn((
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_translation(transform.translation.truncate().extend(0.0) + Vec3::new(x, y, z))),
                Mesh2dHandle(bee_mesh.clone()),
                Bee {
                    kind: BeeKind::Baby,
                    time_alive: 0.0,
                },
                NavigationTarget::None,
                MoveToNavigationTargetBehaviour,
                Velocity::default(),
                VelocityOriented,
                MaxSpeed {value: 64.0},
            ));
        }

        if !nexus.queen_spawned {
            nexus.queen_spawned = true;
            commands.spawn((
                VisibilityBundle::default(),
                TransformBundle::from_transform(Transform::from_translation(transform.translation.truncate().extend(1.0))),
                Mesh2dHandle(bee_mesh.clone()),
                Bee {
                    kind: BeeKind::Queen,
                    time_alive: 0.0,
                },
                NavigationTarget::None,
                MoveToNavigationTargetBehaviour,
                Velocity::default(),
                VelocityOriented,
                MaxSpeed {value: 32.0},
            ));
        }


    }
}
