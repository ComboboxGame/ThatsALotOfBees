use super::BeeMaterial;

use bevy::prelude::*;

use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, EnumIter, Component)]
pub enum BeeKind {
    #[default]
    Baby,
    Regular,
    Worker,
    Builder,
    Defender,
    Queen,
}

#[derive(Component)]
pub struct LivingCreature {
    pub time_alive: f32,
    pub health: u32,
    pub damage: u32,
}

impl From<BeeKind> for LivingCreature {
    fn from(value: BeeKind) -> Self {
        match value {
            BeeKind::Baby => LivingCreature {
                time_alive: 0.0,
                health: 1,
                damage: 0,
            },
            BeeKind::Regular => LivingCreature {
                time_alive: 0.0,
                health: 2,
                damage: 1,
            },
            BeeKind::Worker => todo!(),
            BeeKind::Builder => todo!(),
            BeeKind::Defender => LivingCreature {
                time_alive: 0.0,
                health: 4,
                damage: 2,
            },
            BeeKind::Queen => LivingCreature {
                time_alive: 0.0,
                health: 60,
                damage: 2,
            },
        }
    }
}

pub fn update_bee_material_system(
    mut commands: Commands,
    mut bees: Query<(Entity, &BeeKind), Changed<BeeKind>>,
    mut materials: ResMut<Assets<BeeMaterial>>,
) {
    for (e, bee) in bees.iter_mut() {
        commands.entity(e).insert(materials.add(bee.clone().into()));
    }
}
