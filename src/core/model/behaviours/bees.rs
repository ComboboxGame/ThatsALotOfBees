use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    core::{BeeType, HiveMap, LivingCreature, NavigationResult, NavigationTarget, EnemyType},
    utils::FlatProvider,
};

use super::UniversalBehaviour;

#[derive(Component, Default)]
pub struct BabyBee {}

pub const REGULAR_ATTACK_RADIUS: f32 = 22.0;
pub const QUEEN_ATTACK_RADIUS: f32 = 24.0;

pub fn bee_behaviour_system(
    mut bees: Query<
        (
            &mut BeeType,
            &mut LivingCreature,
            &mut UniversalBehaviour,
            &Transform,
            &mut NavigationTarget,
            &NavigationResult,
        ),
        Without<EnemyType>,
    >,
    mut wasps: Query<(Entity, &mut LivingCreature, &Transform), With<EnemyType>>,
    map: Res<HiveMap>,
    mut rng: Local<Option<StdRng>>,
) {
    if !map.ready {
        return;
    }

    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let mut rng = rng.as_mut().unwrap();

    for (mut bee, mut living_creature, mut behaviour ,transform, mut target, result) in bees.iter_mut() {
        match *bee {
            BeeType::Baby => {
                // ================ BABY BEHAVIOUR START ===============
                /*if result.is_reached() {
                    // Go somewhere...
                    if let Some(pos) =
                        generate_new_target_point(&map, &mut rng, transform.flat(), 120.0, 45.0)
                    {
                        *target = NavigationTarget::Position(pos);
                    }
                }*/

                if living_creature.time_alive > 12.0 {
                    *bee = BeeType::Regular;
                    *living_creature = LivingCreature::from(BeeType::Regular);
                    *behaviour = UniversalBehaviour::from(BeeType::Regular);
                }
                // ================ BABY BEHAVIOUR END ===============
            },
            _ => {}

            /*BeeType::Regular => {
                // ================ REGULAR BEHAVIOUR START ===============
                if living_creature.time_since_last_damage_taken < 0.1 {
                    if let NavigationTarget::Entity(_, _) = *target {
                        // do nothing, already atacking
                    } else {
                        for (e, _, t) in wasps.iter() {
                            let p = t.flat();
                            let me = transform.flat();
                            if p.distance(me) < REGULAR_ATTACK_RADIUS * 1.6 {
                                *target = NavigationTarget::Entity(e, REGULAR_ATTACK_RADIUS);
                                break;
                            }
                        }
                    }
                }

                if result.is_reached() {
                    match *target {
                        NavigationTarget::None | NavigationTarget::Position(_) => {
                            // Go somewhere...
                            if let Some(pos) = generate_new_target_point(
                                &map,
                                &mut rng,
                                transform.flat(),
                                220.0,
                                90.0,
                            ) {
                                *target = NavigationTarget::Position(pos);
                            }
                        }
                        NavigationTarget::Entity(e, _) => {
                            if living_creature.can_attack() {
                                if let Ok((_, mut wasp_creature, t)) = wasps.get_mut(e) {
                                    living_creature
                                        .attack(&mut wasp_creature, t.flat() - transform.flat());
                                }
                            }
                        }
                    }
                }
                // ================ REGULAR BEHAVIOUR END ===============
            }

            BeeType::Worker => todo!(),

            BeeType::Builder => todo!(),

            BeeType::Defender => todo!(),

            BeeType::Queen => {
                // ================ QUEEN BEHAVIOUR START ===============

                if living_creature.time_since_last_damage_taken < 0.1 {
                    if let NavigationTarget::Entity(_, _) = *target {
                        // do nothing, already atacking
                    } else {
                        for (e, _, t) in wasps.iter() {
                            let p = t.flat();
                            let me = transform.flat();
                            if p.distance(me) < QUEEN_ATTACK_RADIUS * 1.6 {
                                *target = NavigationTarget::Entity(e, QUEEN_ATTACK_RADIUS);
                                break;
                            }
                        }
                    }
                }

                if result.is_reached() {
                    match *target {
                        NavigationTarget::None | NavigationTarget::Position(_) => {
                            // Go somewhere...
                            if let Some(pos) = generate_new_target_point(
                                &map,
                                &mut rng,
                                transform.flat(),
                                100.0,
                                50.0,
                            ) {
                                *target = NavigationTarget::Position(pos);
                            }
                        }
                        NavigationTarget::Entity(e, _) => {
                            if living_creature.can_attack() {
                                if let Ok((_, mut wasp_creature, t)) = wasps.get_mut(e) {
                                    living_creature
                                        .attack(&mut wasp_creature, t.flat() - transform.flat());
                                }
                            }
                        }
                    }
                }
                // ================ QUEEN BEHAVIOUR END ===============
            }*/
        }
    }
}

pub fn generate_new_target_point(
    map: &HiveMap,
    rng: &mut StdRng,
    current_position: Vec2,
    area_radius: f32,
    min_distance: f32,
) -> Option<Vec2> {
    for _ in 0..16 {
        let x = rng.gen_range(-area_radius..area_radius);
        let y = rng.gen_range(-area_radius..area_radius);
        let pos = Vec2::new(x, y);
        if pos.length_squared() > area_radius.powi(2)
            || map.get_obstruction(pos) > 0.0
            || current_position.distance_squared(pos) < min_distance.powi(2)
        {
            continue;
        }

        return Some(pos);
    }

    None
}
