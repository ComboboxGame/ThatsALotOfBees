use bevy::{prelude::*, utils::petgraph::visit::EdgeRef};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{
    core::{
        BeeType, HiveMap, LivingCreature, NavigationResult, NavigationTarget, EnemyType,
    },
    utils::FlatProvider,
};

#[derive(Component, Default)]
pub struct BabyBee {}

pub const WASP_ATTACK_RADIUS: f32 = 22.0;

pub fn wasp_behaviour_system(
    mut wasps: Query<
        (
            &EnemyType,
            &mut LivingCreature,
            &Transform,
            &mut NavigationTarget,
            &NavigationResult,
        ),
        Without<BeeType>,
    >,
    mut bees: Query<(Entity, &mut LivingCreature, &Transform), With<BeeType>>,
    mut rng: Local<Option<StdRng>>,
) {
    if rng.is_none() {
        *rng = Some(StdRng::seed_from_u64(0));
    }
    let rng = rng.as_mut().unwrap();

    for (wasp, mut wasp_creature, transform, mut target, result) in wasps.iter_mut()
    {
        /*match *wasp {
            EnemyType::Wasp => {
                // ================ REGULAR BEHAVIOUR START ===============
                if *target == NavigationTarget::None {
                    let mut nearest = None;
                    let mut nearest_dist = 1e9;
                    for (e, creature, t) in bees.iter() {
                        let dist = t.flat().distance(transform.flat());
                        if !creature.is_dead() && dist < nearest_dist {
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
        }*/
    }
}
