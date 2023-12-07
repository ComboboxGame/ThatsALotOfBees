use bevy::{prelude::*, utils::petgraph::visit::EdgeRef};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    core::{
        BeeKind, HiveMap, LivingCreature, MaxSpeed, NavigationResult, NavigationTarget, WaspKind,
    },
    utils::FlatProvider,
};

#[derive(Component, Default)]
pub struct BabyBee {}

pub const WASP_ATTACK_RADIUS: f32 = 22.0;

pub fn wasp_behaviour_system(
    mut wasps: Query<
        (
            &WaspKind,
            &mut LivingCreature,
            &Transform,
            &mut NavigationTarget,
            &NavigationResult,
            &mut MaxSpeed,
        ),
        Without<BeeKind>,
    >,
    mut bees: Query<(Entity, &mut LivingCreature, &Transform), With<BeeKind>>,
    mut rng: Local<Option<StdRng>>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    for (wasp, mut wasp_creature, transform, mut target, result, mut max_speed) in wasps.iter_mut()
    {
        match *wasp {
            WaspKind::Regular => {
                // ================ REGULAR BEHAVIOUR START ===============
                if *target == NavigationTarget::None {
                    let mut nearest = None;
                    let mut nearest_dist = 1e9;
                    for (e, _, t) in bees.iter() {
                        let dist = t.translation.truncate().distance(transform.flat());
                        if dist < nearest_dist {
                            nearest_dist = dist;
                            nearest = Some(e);
                        }
                    }
                    if let Some(e) = nearest {
                        *target = NavigationTarget::Entity(e, WASP_ATTACK_RADIUS);
                    }
                } else if let NavigationTarget::Entity(e, _) = *target {
                    if result.is_reached() && wasp_creature.can_attack() {
                        if let Ok((_, mut bee_creature, t)) = bees.get_mut(e) {
                            wasp_creature.attack(&mut bee_creature, t.flat() - transform.flat());
                        }
                    }
                }
                // ================ REGULAR BEHAVIOUR END ===============
            }
        }
    }
}
