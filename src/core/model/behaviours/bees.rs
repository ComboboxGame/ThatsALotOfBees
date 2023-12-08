use bevy::prelude::*;

use crate::core::{BeeType, EnemyType, LivingCreature, CurrencyGainPerMinute, UniversalMaterial};

use super::UniversalBehaviour;

pub fn baby_behaviour_system(
    mut bees: Query<
        (
            &mut BeeType,
            &mut LivingCreature,
            &mut UniversalBehaviour,
            &mut CurrencyGainPerMinute,
            &Handle<UniversalMaterial>
        ),
        Without<EnemyType>,
    >,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    time: Res<Time>,
) {
    for (mut bee, mut living_creature, mut behaviour, mut gain, material) in bees.iter_mut() {
        match *bee {
            BeeType::Baby => {
                if living_creature.time_alive > 12.0 {
                    *bee = BeeType::Regular;
                    *living_creature = LivingCreature::from(BeeType::Regular);
                    *behaviour = UniversalBehaviour::from(BeeType::Regular);
                    *gain = CurrencyGainPerMinute::from(BeeType::Regular);
                    if let Some(material) = materials.get_mut(material) {
                        material.props.upgrade_time = time.elapsed_seconds();
                    }
                }
            }
            _ => {}
        }
    }
}
