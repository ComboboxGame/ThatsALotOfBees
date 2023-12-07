use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::core::{BeeKind, HiveMap, LivingCreature, NavigationResult, NavigationTarget};

#[derive(Component, Default)]
pub struct BabyBee {}

pub fn bee_behaviour_system(
    mut bees: Query<(
        &mut BeeKind,
        &LivingCreature,
        &Transform,
        &mut NavigationTarget,
        &NavigationResult,
    )>,
    map: Res<HiveMap>,
    mut rng: Local<Option<StdRng>>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    for (mut bee, living_creature, transform, mut target, result) in bees.iter_mut() {
        match *bee {
            BeeKind::Baby => {
                // ================ BABY BEHAVIOUR START ===============
                if result.is_reached() {
                    for _ in 0..16 {
                        const AREA_RADIUS: f32 = 120.0;
                        const MIN_DISTNACE_TO_NEW_TARGET: f32 = 48.0;
                        let x = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                        let y = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                        if x * x + y * y > AREA_RADIUS * AREA_RADIUS {
                            continue;
                        }
                        if map.get_obstruction_xy(x, y) > 0.0 {
                            continue;
                        }
                        let pos = Vec2::new(x, y);
                        if transform.translation.truncate().distance(pos)
                            < MIN_DISTNACE_TO_NEW_TARGET
                        {
                            continue;
                        }

                        *target = NavigationTarget::Position(pos);
                    }
                }

                if living_creature.time_alive > 20.0 {
                    *bee = BeeKind::Regular;
                }
                // ================ BABY BEHAVIOUR END ===============
            }

            BeeKind::Regular => {
                // ================ REGULAR BEHAVIOUR START ===============
                if result.is_reached() {
                    for _ in 0..16 {
                        const AREA_RADIUS: f32 = 220.0;
                        const MIN_DISTNACE_TO_NEW_TARGET: f32 = 90.0;
                        let x = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                        let y = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                        if x * x + y * y > AREA_RADIUS * AREA_RADIUS {
                            continue;
                        }
                        if map.get_obstruction_xy(x, y) > 0.0 {
                            continue;
                        }
                        let pos = Vec2::new(x, y);
                        if transform.translation.truncate().distance(pos)
                            < MIN_DISTNACE_TO_NEW_TARGET
                        {
                            continue;
                        }

                        *target = NavigationTarget::Position(pos);
                    }
                }
                // ================ REGULAR BEHAVIOUR END ===============
            }

            BeeKind::Worker => todo!(),

            BeeKind::Builder => todo!(),

            BeeKind::Defender => todo!(),

            BeeKind::Queen => {
                // ================ QUEEN BEHAVIOUR START ===============
                if result.is_reached() {
                    for _ in 0..16 {
                        const AREA_RADIUS: f32 = 100.0;
                        const MIN_DISTNACE_TO_NEW_TARGET: f32 = 50.0;
                        let x = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                        let y = rng.gen_range(-AREA_RADIUS..AREA_RADIUS);
                        if x * x + y * y > AREA_RADIUS * AREA_RADIUS {
                            continue;
                        }
                        if map.get_obstruction_xy(x, y) > 0.0 {
                            continue;
                        }
                        let pos = Vec2::new(x, y);
                        if transform.translation.truncate().distance(pos)
                            < MIN_DISTNACE_TO_NEW_TARGET
                        {
                            continue;
                        }

                        *target = NavigationTarget::Position(pos);
                    }
                }
                // ================ QUEEN BEHAVIOUR END ===============
            }
        }
    }
}
