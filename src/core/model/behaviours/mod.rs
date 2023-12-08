use bevy::prelude::*;

mod bees;
mod buildings;
mod universal_behaviour;

use bees::*;
use buildings::*;

pub use universal_behaviour::*;

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, universal_behaviour_system);
        app.add_systems(Update, baby_behaviour_system);
        app.add_systems(Update, fight_system);
        app.add_systems(Update, nexus_system);
    }
}
