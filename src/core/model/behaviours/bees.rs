use bevy::prelude::*;

use crate::core::{BeeType, EnemyType, LivingCreature};

use super::UniversalBehaviour;

pub fn baby_behaviour_system(
    mut bees: Query<
        (
            &mut BeeType,
            &mut LivingCreature,
            &mut UniversalBehaviour,
            &Transform,
        ),
        Without<EnemyType>,
    >,
) {
    for (mut bee, mut living_creature, mut behaviour, _transform) in bees.iter_mut() {
        match *bee {
            BeeType::Baby => {
                if living_creature.time_alive > 12.0 {
                    *bee = BeeType::Regular;
                    *living_creature = LivingCreature::from(BeeType::Regular);
                    *behaviour = UniversalBehaviour::from(BeeType::Regular);
                }
            }
            _ => {}
        }
    }
}
