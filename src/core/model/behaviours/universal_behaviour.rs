use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    core::{
        BeeType, EnemyType, Faction, HiveMap, LivingCreature, NavigationResult, NavigationTarget,
        RigidBody,
    },
    utils::FlatProvider,
};

#[derive(Component)]
pub struct UniversalBehaviour {
    pub max_wonder_distance_to_hive: f32,
    pub min_wonder_distance_to_hive: f32,
    pub min_wonder_distance: f32,

    pub enemy_attack_distance_to_hive: f32,
    pub enemy_attack_radius: f32,
    pub enemy_attack_radius_if_alerted: f32,
    pub alert_distance: f32,

    pub run_away_radius: f32,
    pub min_healthpoints_before_run_away: i32,

    pub time_since_last_refresh: f32,
}

// Navigation target:
// Point - go to point
// Entity - go to entity
//

impl From<BeeType> for UniversalBehaviour {
    fn from(value: BeeType) -> Self {
        match value {
            BeeType::Baby => UniversalBehaviour {
                max_wonder_distance_to_hive: 100.0,
                min_wonder_distance_to_hive: 0.0,
                min_wonder_distance: 50.0,
                enemy_attack_distance_to_hive: 0.0,
                enemy_attack_radius: 0.0,
                enemy_attack_radius_if_alerted: 0.0,
                alert_distance: 0.0,
                run_away_radius: 40.0,
                min_healthpoints_before_run_away: 100,
                time_since_last_refresh: 0.0,
            },
            BeeType::Regular => UniversalBehaviour {
                max_wonder_distance_to_hive: 150.0,
                min_wonder_distance_to_hive: 40.0,
                min_wonder_distance: 60.0,
                enemy_attack_distance_to_hive: 1000.0,
                enemy_attack_radius: 40.0,
                enemy_attack_radius_if_alerted: 40.0,
                alert_distance: 0.0,
                run_away_radius: 0.0,
                min_healthpoints_before_run_away: 0,
                time_since_last_refresh: 0.0,
            },
            BeeType::Worker => UniversalBehaviour {
                max_wonder_distance_to_hive: 340.0,
                min_wonder_distance_to_hive: 110.0,
                min_wonder_distance: 100.0,
                enemy_attack_distance_to_hive: 0.0,
                enemy_attack_radius: 0.0,
                enemy_attack_radius_if_alerted: 0.0,
                alert_distance: 0.0,
                run_away_radius: 60.0,
                min_healthpoints_before_run_away: 100,
                time_since_last_refresh: 0.0,
            },
            BeeType::Builder => todo!(),
            BeeType::Defender => UniversalBehaviour {
                max_wonder_distance_to_hive: 220.0,
                min_wonder_distance_to_hive: 110.0,
                min_wonder_distance: 80.0,
                enemy_attack_distance_to_hive: 1000.0,
                enemy_attack_radius: 100.0,
                enemy_attack_radius_if_alerted: 1000.0,
                alert_distance: 1000.0,
                run_away_radius: 0.0,
                min_healthpoints_before_run_away: 0,
                time_since_last_refresh: 0.0,
            },
            BeeType::Queen => UniversalBehaviour {
                max_wonder_distance_to_hive: 110.0,
                min_wonder_distance_to_hive: 0.0,
                min_wonder_distance: 50.0,
                enemy_attack_distance_to_hive: 120.0,
                enemy_attack_radius: 1000.0,
                enemy_attack_radius_if_alerted: 1000.0,
                alert_distance: 0.0,
                run_away_radius: 40.0,
                min_healthpoints_before_run_away: 10,
                time_since_last_refresh: 0.0,
            },
        }
    }
}

impl From<EnemyType> for UniversalBehaviour {
    fn from(value: EnemyType) -> Self {
        match value {
            EnemyType::Wasp | EnemyType::Birb => UniversalBehaviour {
                max_wonder_distance_to_hive: 200.0,
                min_wonder_distance_to_hive: 20.0,
                min_wonder_distance: 90.0,
                enemy_attack_distance_to_hive: 5000.0,
                enemy_attack_radius: 5000.0,
                enemy_attack_radius_if_alerted: 5000.0,
                alert_distance: 0.0,
                run_away_radius: 0.0,
                min_healthpoints_before_run_away: 0,
                time_since_last_refresh: 0.0,
            },
        }
    }
}

pub fn fight_system(
    mut bees: Query<
        (
            &mut LivingCreature,
            &RigidBody,
            &Transform,
            &NavigationTarget,
        ),
        Without<EnemyType>,
    >,
    mut enemies: Query<
        (
            &mut LivingCreature,
            &RigidBody,
            &Transform,
            &NavigationTarget,
        ),
        With<EnemyType>,
    >,
) {
    for (mut bee_creature, _, bee_transform, bee_target) in bees.iter_mut() {
        if let NavigationTarget::Entity(e, _) = bee_target {
            if let Ok((mut enemy_creature, enemy_rb, enemy_transform, _)) = enemies.get_mut(*e) {
                let dist_sqr = bee_transform
                    .flat()
                    .distance_squared(enemy_transform.flat());
                let bee_attack_radius = bee_creature.attack_radius + enemy_rb.radius;

                if dist_sqr < bee_attack_radius.powi(2) && bee_creature.can_attack() {
                    bee_creature.attack(
                        &mut enemy_creature,
                        enemy_transform.flat() - bee_transform.flat(),
                    );
                }
            }
        }
    }

    for (mut enemy_creature, _, enemy_transform, enemy_target) in enemies.iter_mut() {
        if let NavigationTarget::Entity(e, _) = enemy_target {
            if let Ok((mut bee_creature, bee_rb, bee_transform, _)) = bees.get_mut(*e) {
                let dist_sqr = enemy_transform
                    .flat()
                    .distance_squared(bee_transform.flat());
                let enemy_attack_radius = enemy_creature.attack_radius + bee_rb.radius;

                if dist_sqr < enemy_attack_radius.powi(2) && enemy_creature.can_attack() {
                    enemy_creature.attack(
                        &mut bee_creature,
                        bee_transform.flat() - enemy_transform.flat(),
                    );
                }
            }
        }
    }
}

pub fn universal_behaviour_system(
    mut behaviours: Query<(
        Entity,
        &mut UniversalBehaviour,
        &LivingCreature,
        &Transform,
        &Faction,
        &mut RigidBody,
        &mut NavigationTarget,
        &NavigationResult,
    )>,
    all: Query<(Entity, &LivingCreature, &Transform, &Faction)>,
    time: Res<Time>,
    map: Res<HiveMap>,
    mut rng: Local<Option<StdRng>>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    for (_e, mut behaviour, creature, transform, faction, mut rb, mut navigation, result) in
        behaviours.iter_mut()
    {
        if creature.is_dead() {
            continue;
        }

        behaviour.time_since_last_refresh += time.delta_seconds();

        let should_refresh = match *navigation {
            NavigationTarget::None => true,
            NavigationTarget::Position(_) => {
                result.is_reached() || behaviour.time_since_last_refresh > 1.0
            }
            NavigationTarget::Entity(_, _) => behaviour.time_since_last_refresh > 1.0,
        };

        if !should_refresh {
            continue;
        }

        behaviour.time_since_last_refresh = 0.0;

        let mut nearest_enemy = None;
        let mut nearest_enemy_dist_sqr = 1e18;

        let mut is_alert = false;

        for (other_e, other_creature, other_t, other_faction) in all.iter() {
            if other_creature.is_dead() {
                continue;
            }

            let dist_sqr = other_t.flat().distance_squared(transform.flat());
            if other_faction != faction {
                if dist_sqr < nearest_enemy_dist_sqr
                    && other_t.flat().length_squared()
                        < behaviour.enemy_attack_distance_to_hive.powi(2)
                {
                    nearest_enemy_dist_sqr = dist_sqr;
                    nearest_enemy = Some(other_e);
                }
            } else {
                if dist_sqr < behaviour.alert_distance.powi(2)
                    && other_creature.time_since_last_damage_taken < 1.0
                {
                    is_alert = true;
                }
            }
        }

        let enemy_attack_radius = if is_alert {

            println!("Alert!");
            behaviour.enemy_attack_radius_if_alerted
        } else {
            behaviour.enemy_attack_radius
        };

        if nearest_enemy_dist_sqr.sqrt() < enemy_attack_radius && nearest_enemy.is_some() {
            // check if should run???

            // go to enemy!!!!
            *navigation = NavigationTarget::Entity(nearest_enemy.unwrap(), creature.attack_radius);
        } else {
            let refresh_wonder = if let NavigationTarget::Position(_) = *navigation {
                result.is_reached() || rb.stuck_tick > 5
            } else {
                true
            };

            if refresh_wonder {
                rb.stuck_tick = 0;
                // wonder around then...
                for _ in 0..16 {
                    let x = rng.gen_range(
                        -behaviour.max_wonder_distance_to_hive
                            ..behaviour.max_wonder_distance_to_hive,
                    );
                    let y = rng.gen_range(
                        -behaviour.max_wonder_distance_to_hive
                            ..behaviour.max_wonder_distance_to_hive,
                    );
                    let pos = Vec2::new(x, y);
                    if pos.length_squared() > behaviour.max_wonder_distance_to_hive.powi(2)
                        || pos.length_squared() < behaviour.min_wonder_distance_to_hive.powi(2)
                        || map.get_obstruction(pos) > 0.0
                        || transform.flat().distance_squared(pos)
                            < behaviour.min_wonder_distance.powi(2)
                    {
                        continue;
                    }

                    *navigation = NavigationTarget::Position(pos);
                    break;
                }
            }
        }
    }
}
