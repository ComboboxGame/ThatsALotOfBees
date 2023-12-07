use bevy::prelude::*;

use crate::{
    core::{HiveMap, NavigationTarget},
    utils::FlatProvider,
};

use super::LivingCreature;

#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec2,
}

#[derive(Component, Default)]
pub struct MaxSpeed {
    pub value: f32,
}

#[derive(Component)]
pub struct VelocityOriented;

pub fn movement_system(
    mut agents: Query<(&Velocity, &mut Transform, Option<&LivingCreature>)>,
    time: Res<Time>,
    map: Res<HiveMap>,
) {
    if !map.ready {
        return;
    }
    
    for (velocity, mut transform, maybe_creature) in agents.iter_mut() {
        if let Some(creature) = maybe_creature {
            if creature.is_dead() {
                continue;
            }
        }

        let next = transform.flat() + velocity.value * time.delta_seconds();
        if map.get_obstruction(next) < 0.9 {
            transform.translation += velocity.value.extend(0.0) * time.delta_seconds();
        } else {
            let next_x = transform.translation.x + velocity.value.x * time.delta_seconds();
            let next_y = transform.translation.y + velocity.value.y * time.delta_seconds();

            if map.get_obstruction_xy(next_x, transform.translation.y) < 0.9 {
                transform.translation +=
                    Vec3::new(velocity.value.x, 0.0, 0.0) * time.delta_seconds();
            }
            if map.get_obstruction_xy(transform.translation.x, next_y) < 0.9 {
                transform.translation +=
                    Vec3::new(0.0, velocity.value.y, 0.0) * time.delta_seconds();
            }
        }
    }
}

pub fn movement_orientation_system(
    mut agents: Query<
        (&Velocity, &mut Transform, Option<&NavigationTarget>),
        With<VelocityOriented>,
    >,
    transforms: Query<&GlobalTransform, With<LivingCreature>>,
) {
    for (velocity, mut transform, maybe_target) in agents.iter_mut() {
        let delta = if let Some(NavigationTarget::Entity(e, target_range)) = maybe_target {
            if let Ok(t) = transforms.get(*e) {
                if t.flat().distance_squared(transform.flat()) < (target_range * 2.5).powi(2) {
                    t.flat() - transform.flat()
                } else {
                    velocity.value
                }
            } else {
                velocity.value
            }
        } else {
            velocity.value
        };

        if delta.length() > 0.01 {
            if transform.scale.x > 0.0 {
                // positive orientation already
                if delta.normalize().x > 0.1 {
                    transform.scale.x = -1.0;
                }
            } else {
                // negative orientation already
                if delta.normalize().x < -0.1 {
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
