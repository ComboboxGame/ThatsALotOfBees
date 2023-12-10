use bevy::prelude::*;

use crate::{
    core::{HiveMap, NavigationResult, NavigationTarget},
    utils::FlatProvider,
};

use super::{BeeType, EnemyType, LivingCreature, GameInfo};

#[derive(Component, Default)]
pub struct RigidBody {
    pub radius: f32,
    pub velocity: Vec2,
    pub pseudo_velocity: Vec2,
    pub max_valocity: f32,
    pub max_acceleartion: f32,
    pub stuck_tick: u32,
}

impl From<BeeType> for RigidBody {
    fn from(value: BeeType) -> Self {
        match value {
            BeeType::Baby => RigidBody {
                radius: 6.0,
                max_valocity: 40.0,
                max_acceleartion: 250.0,
                ..Default::default()
            },
            BeeType::Regular => RigidBody {
                radius: 8.0,
                max_valocity: 40.0,
                max_acceleartion: 250.0,
                ..Default::default()
            },
            BeeType::Worker(lvl) => RigidBody {
                radius: 8.0,
                max_valocity: 38.0,
                max_acceleartion: 250.0,
                ..Default::default()
            },
            BeeType::Defender(lvl) => RigidBody {
                radius: 8.0,
                max_valocity: 40.0,
                max_acceleartion: 250.0,
                ..Default::default()
            },
            BeeType::Queen => RigidBody {
                radius: 10.0,
                max_valocity: 40.0,
                max_acceleartion: 250.0,
                ..Default::default()
            },
        }
    }
}

impl From<EnemyType> for RigidBody {
    fn from(value: EnemyType) -> Self {
        match value {
            EnemyType::Wasp(lvl) => RigidBody {
                radius: 9.0,
                max_valocity: 45.0,
                max_acceleartion: 300.0,
                ..Default::default()
            },
            EnemyType::Birb(lvl) => RigidBody {
                radius: 13.0,
                max_valocity: 45.0,
                max_acceleartion: 350.0,
                ..Default::default()
            },
            EnemyType::Bumble(lvl) => RigidBody {
                radius: 13.0,
                max_valocity: 35.0,
                max_acceleartion: 200.0,
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
pub struct SmartOrientation;

pub fn integration_system(
    mut rigid_bodies: Query<(&mut RigidBody, &mut Transform, Option<&LivingCreature>)>,
    time: Res<Time>,
    game: Res<GameInfo>,
    map: Res<HiveMap>,
) {
    if game.paused {
        return;
    }
    if !map.ready {
        return;
    }

    for (mut rb, mut transform, maybe_creature) in rigid_bodies.iter_mut() {
        if let Some(creature) = maybe_creature {
            if creature.is_dead() {
                //continue;
            }
        }

        let total_velocity = rb.velocity + rb.pseudo_velocity;
        let current_pos = transform.flat();
        let next_pos = current_pos + total_velocity * time.delta_seconds();

        rb.pseudo_velocity = Vec2::ZERO;

        if map.get_obstruction(next_pos) == 0.0 || map.get_obstruction(current_pos) > 0.0 {
            // No obstruction, integrate velocity normally
            transform.translation.x = next_pos.x;
            transform.translation.y = next_pos.y;
        } else {
            if map.get_obstruction_xy(current_pos.x, next_pos.y) == 0.0 {
                // Integrate along y only
                transform.translation.y = next_pos.y;
                rb.velocity.x = 0.0;
                rb.stuck_tick += 1;
            } else if map.get_obstruction_xy(next_pos.x, current_pos.y) == 0.0 {
                // Integrate along x only
                transform.translation.x = next_pos.x;
                rb.velocity.y = 0.0;
                rb.stuck_tick += 1;
            } else {
                // Corner?
                rb.velocity = Vec2::ZERO;
                rb.stuck_tick += 1;
            }
        }
    }
}

pub fn collision_system(
    mut a_rigid_bodies: Query<(&mut RigidBody, &Transform, &LivingCreature), With<BeeType>>,
    mut b_rigid_bodies: Query<(&mut RigidBody, &Transform, &LivingCreature), Without<BeeType>>,
    time: Res<Time>,
    game: Res<GameInfo>,
    _map: Res<HiveMap>,
) {
    if game.paused {
        return;
    }
    for (mut arb, at, alc) in a_rigid_bodies.iter_mut() {
        if alc.is_dead() {
            continue;
        }
        for (mut brb, bt, blc) in b_rigid_bodies.iter_mut() {
            let dist_sqr = at.flat().distance_squared(bt.flat());
            let total_radius = arb.radius + brb.radius;
            if blc.is_dead() || dist_sqr > total_radius.powi(2) {
                continue;
            }
            let penetration = total_radius - dist_sqr.sqrt();

            let ab = (bt.flat() - at.flat()).normalize();

            if penetration > 0.01 {
                arb.pseudo_velocity -= penetration / time.delta_seconds() * 0.2 * ab;
                brb.pseudo_velocity += penetration / time.delta_seconds() * 0.2 * ab;
            }


            let m1 = arb.radius.powi(2);
            let m2 = brb.radius.powi(2);

            let delta_velocity = ab.dot(arb.velocity - brb.velocity);

            if delta_velocity > 0.0 {
                let effective_mass = 1.0 / (1.0 / m1 + 1.0 / m2);
                let delta_impulse = delta_velocity * effective_mass;
                arb.velocity -= delta_impulse * ab / m1;
                brb.velocity += delta_impulse * ab / m2;
            }
        }
    }
}

pub fn orientation_system(
    mut agents: Query<
        (&RigidBody, &mut Transform, Option<&NavigationTarget>),
        With<SmartOrientation>,
    >,
    transforms: Query<&GlobalTransform, With<LivingCreature>>,
) {
    for (rb, mut transform, maybe_target) in agents.iter_mut() {
        let delta = if let Some(NavigationTarget::Entity(e, target_range)) = maybe_target {
            if let Ok(t) = transforms.get(*e) {
                if t.flat().distance_squared(transform.flat()) < (target_range * 2.5).powi(2) {
                    t.flat() - transform.flat()
                } else {
                    rb.velocity
                }
            } else {
                rb.velocity
            }
        } else {
            rb.velocity
        };

        if delta.length() > 0.01 {
            if transform.scale.x > 0.0 {
                // positive orientation already
                if delta.normalize().x > 0.05 {
                    transform.scale.x = -1.0;
                }
            } else {
                // negative orientation already
                if delta.normalize().x < -0.05 {
                    transform.scale.x = 1.0;
                }
            }

            let angle = if delta.x < 0.0 {
                f32::atan2(-delta.y, -delta.x)
            } else {
                f32::atan2(delta.y, delta.x)
            };
            transform.rotation =
                Quat::from_axis_angle(Vec3::Z, (angle.clamp(-1.0, 1.0) / 0.7).sin() * 0.7);
        }
    }
}

#[derive(Component)]
pub struct MoveToNavigationTargetBehaviour;

pub fn move_to_target_system(
    mut agents: Query<
        (&mut RigidBody, &Transform, &NavigationResult, &LivingCreature),
        With<MoveToNavigationTargetBehaviour>,
    >,
    time: Res<Time>,
    game: Res<GameInfo>,
) {
    if game.paused {
        return;
    }
    for (mut rb, transform, result, creature) in agents.iter_mut() {
        if creature.is_dead() {
            continue;
        }
        
        let target_velocity = result.get_direction(transform.flat()) * rb.max_valocity;

        let mut delta_velocity = target_velocity - rb.velocity;

        let max_delta_velocity = rb.max_acceleartion * time.delta_seconds();

        if delta_velocity.length() > max_delta_velocity {
            delta_velocity = delta_velocity.normalize() * max_delta_velocity;
        }

        rb.velocity += delta_velocity;
    }
}
