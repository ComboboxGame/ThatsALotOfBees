use bevy::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::core::{WaspKind, HiveMap, LivingCreature, NavigationResult, NavigationTarget, BeeKind};

#[derive(Component, Default)]
pub struct BabyBee {}

pub fn wasp_behaviour_system(
    mut wasps: Query<(
        &WaspKind,
        &LivingCreature,
        &Transform,
        &mut NavigationTarget,
        &NavigationResult,
    ), Without<BeeKind>>,
    bees: Query<(Entity, &Transform), With<BeeKind>>,
    mut rng: Local<Option<StdRng>>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    for (wasp, _, transform, mut target, result) in wasps.iter_mut() {
        match *wasp {
            WaspKind::Regular => {
                // ================ REGULAR BEHAVIOUR START ===============
                if *target == NavigationTarget::None {
                    let mut nearest = None;
                    let mut nearest_dist = 1e9;
                    for (e, t) in bees.iter() {
                        let dist = t.translation.truncate().distance(transform.translation.truncate());
                        if dist < nearest_dist {
                            nearest_dist = dist;
                            nearest = Some(e);
                        }
                    }
                    if let Some(e) = nearest {
                        *target = NavigationTarget::Entity(e, 20.0);
                    }
                }
                // ================ REGULAR BEHAVIOUR END ===============
            }
        }
    }
}
