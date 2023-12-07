use bevy::prelude::*;

mod bees;
mod nexus;
mod wasps;

use nexus::*;

use self::{bees::bee_behaviour_system, wasps::wasp_behaviour_system};

use super::{BeeKind, Building, BuildingKind, LivingCreature};

pub fn living_creature_system(mut creatures: Query<&mut LivingCreature>, time: Res<Time>) {
    for mut creature in creatures.iter_mut() {
        creature.time_alive += time.delta_seconds();
    }
}

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, living_creature_system);
        app.add_systems(Update, bee_behaviour_system);
        app.add_systems(Update, wasp_behaviour_system);
        app.add_systems(Update, nexus_system);
    }
}
