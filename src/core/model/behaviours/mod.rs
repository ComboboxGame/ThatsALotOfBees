use bevy::prelude::*;

mod baby;
mod regular;
mod queen;
mod nexus;

use baby::*;
use regular::*;
use nexus::*;
use queen::*;

use super::{Bee, BeeKind, Building, BuildingKind};

pub fn bee_system(mut bees: Query<&mut Bee>, time: Res<Time>) {
    for mut bee in bees.iter_mut() {
        bee.time_alive += time.delta_seconds();
    }
}

pub fn update_behaviours_system(
    mut commands: Commands,
    bees: Query<(Entity, &Bee, Option<&BabyBee>, Option<&QueenBee>)>,
    buildings: Query<(Entity, &Building, Option<&Nexus>)>,
) {
    for (e, bee, is_baby, is_queen) in bees.iter() {
        match bee.kind {
            BeeKind::Baby => {
                if is_baby.is_none() {
                    commands.entity(e).insert(BabyBee::default());
                }
            }
            BeeKind::Regular => {}
            BeeKind::Worker => {}
            BeeKind::Builder => {}
            BeeKind::Defender => {}
            BeeKind::Queen => {
                if is_queen.is_none() {
                    commands.entity(e).insert(QueenBee::default());
                }
            }
        }

        if is_baby.is_some() && bee.kind != BeeKind::Baby {
            commands.entity(e).remove::<BabyBee>();
        }
        if is_queen.is_some() && bee.kind != BeeKind::Queen {
            commands.entity(e).remove::<QueenBee>();
        }
    }

    for (e, building, is_nexus) in buildings.iter() {
        match building.kind {
            BuildingKind::None => {}
            BuildingKind::Nexus => {
                if is_nexus.is_none() {
                    commands.entity(e).insert(Nexus::default());
                }
            }
            BuildingKind::Storage => todo!(),
            BuildingKind::WaxReactor => todo!(),
            BuildingKind::Armory => todo!(),
            BuildingKind::Workshop => todo!(),
            BuildingKind::BuilderAcademy => todo!(),
        }

        if is_nexus.is_some() && building.kind != BuildingKind::Nexus {
            commands.entity(e).remove::<Nexus>();
        }
    }
}

pub struct BehaviourPlugin;

impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_behaviours_system);

        app.add_systems(Update, bee_system);
        app.add_systems(Update, baby_bee_system);
        app.add_systems(Update, queen_bee_system);

        app.add_systems(Update, nexus_system);
    }
}
