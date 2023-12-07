use super::{BeeMaterial, LivingCreature};

use bevy::prelude::*;

use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, EnumIter, Component)]
pub enum WaspKind {
    #[default]
    Regular,
}

impl From<WaspKind> for LivingCreature {
    fn from(value: WaspKind) -> Self {
        match value {
            WaspKind::Regular => LivingCreature {
                time_alive: 0.0,
                health: 10,
                damage: 1,
            },
        }
    }
}

pub fn update_wasp_material_system(
    mut commands: Commands,
    mut wasps: Query<(Entity, &WaspKind), Changed<WaspKind>>,
    mut materials: ResMut<Assets<BeeMaterial>>,
) {
    for (e, wasp) in wasps.iter_mut() {
        commands
            .entity(e)
            .insert(materials.add(wasp.clone().into()));
    }
}
