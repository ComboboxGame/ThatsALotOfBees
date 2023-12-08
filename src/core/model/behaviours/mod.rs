use bevy::prelude::*;

mod bees;
mod nexus;
mod universal_behaviour;

use nexus::*;

pub use universal_behaviour::*;

use crate::core::NavigationTarget;

use self::{bees::baby_behaviour_system, universal_behaviour::universal_behaviour_system};

use super::{UniversalMaterial, LivingCreature, RigidBody};

pub fn living_creature_system(
    mut creatures: Query<(
        Entity,
        &mut LivingCreature,
        Option<&Handle<UniversalMaterial>>,
        Option<&mut RigidBody>,
    )>,
    mut targets: Query<&mut NavigationTarget>,
    time: Res<Time>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
    mut commands: Commands,
) {
    for (e, mut creature, maybe_material, maybe_rb) in creatures.iter_mut() {
        if creature.time_since_last_damage_taken == 0.0 {
            if let Some(material) = maybe_material {
                if let Some(material) = materials.get_mut(material) {
                    material.props.damage_time = time.elapsed_seconds();
                }
            }
        }

        if let Some(mut rb) = maybe_rb {
            rb.velocity += creature.accumulated_push_back * 40.0;
        }

        creature.accumulated_push_back = Vec2::ZERO;

        creature.time_alive += time.delta_seconds();
        creature.time_since_last_attack += time.delta_seconds();
        creature.time_since_last_damage_taken += time.delta_seconds();

        if creature.is_dead() && creature.time_since_last_damage_taken > 0.6 {
            commands.entity(e).despawn();
        }
    }

    // Clear targets to dead living creatures
    for mut target in targets.iter_mut() {
        if let NavigationTarget::Entity(e, _) = *target {
            if let Ok((_, creature, _, _)) = creatures.get(e) {
                if creature.is_dead() {
                    *target = NavigationTarget::None;
                }
            } else {
                *target = NavigationTarget::None;
            }
        }
    }
}

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, living_creature_system);
        app.add_systems(Update, universal_behaviour_system);
        app.add_systems(Update, baby_behaviour_system);
        app.add_systems(Update, fight_system);
        app.add_systems(Update, nexus_system);
    }
}
