use super::UniversalMaterial;

use bevy::prelude::*;
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, Component, EnumIter)]
pub enum BeeType {
    #[default]
    Baby,
    Regular,
    Worker,
    Builder,
    Defender,
    Queen,
}

pub fn update_bee_material_system(
    mut commands: Commands,
    mut bees: Query<(Entity, &BeeType), Changed<BeeType>>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
) {
    for (e, bee) in bees.iter_mut() {
        commands.entity(e).insert(materials.add(bee.clone().into()));
    }
}
