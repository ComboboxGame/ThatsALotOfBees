use super::UniversalMaterial;

use bevy::prelude::*;

use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter, Component)]
pub enum EnemyType {
    Wasp(u32),
    Birb(u32),
    Bumble(u32),
}

pub fn update_wasp_material_system(
    mut commands: Commands,
    mut wasps: Query<(Entity, &EnemyType), Changed<EnemyType>>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
) {
    for (e, wasp) in wasps.iter_mut() {
        commands
            .entity(e)
            .insert(materials.add(wasp.clone().into()));
    }
}
