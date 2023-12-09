use crate::core::{Faction, NavigationResult, NavigationTarget};

use super::{
    CurrencyGainPerMinute, LivingCreature, MoveToNavigationTargetBehaviour, RigidBody,
    SmartOrientation, UniversalBehaviour, UniversalMaterial, BEE_MESH,
};

use bevy::{prelude::*, sprite::Mesh2dHandle};
use rand::{thread_rng, Rng};
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, Component, EnumIter)]
pub enum BeeType {
    #[default]
    Baby,
    Regular,
    Worker(u32),
    Defender(u32),
    Queen,
}

pub const MAX_DEFENDER_LEVEL: u32 = 3;
pub const MAX_WORKER_LEVEL: u32 = 3;

pub fn update_bee_material_system(
    mut commands: Commands,
    mut bees: Query<(Entity, &BeeType, Option<&Handle<UniversalMaterial>>), Changed<BeeType>>,
    mut materials: ResMut<Assets<UniversalMaterial>>,
) {
    for (e, bee, material) in bees.iter_mut() {
        let mut new_material: UniversalMaterial = bee.clone().into();
        if let Some(material) = material {
            if let Some(material) = materials.get(material) {
                new_material.props.upgrade_time = material.props.upgrade_time;
                new_material.props.damage_time = material.props.damage_time;
            }
        }
        commands.entity(e).insert(materials.add(new_material));
    }
}

#[derive(Bundle)]
pub struct BeeBundle {
    visiblity: VisibilityBundle,
    transform: TransformBundle,
    mesh: Mesh2dHandle,
    bee_type: BeeType,
    creature: LivingCreature,
    rigid_body: RigidBody,
    behaviour: UniversalBehaviour,
    target: NavigationTarget,
    result: NavigationResult,
    move_behaviour: MoveToNavigationTargetBehaviour,
    orientation: SmartOrientation,
    faction: Faction,
    gain: CurrencyGainPerMinute,
}

impl From<(BeeType, Vec2)> for BeeBundle {
    fn from(value: (BeeType, Vec2)) -> Self {
        let (bee_type, position) = value;

        BeeBundle {
            visiblity: VisibilityBundle::default(),
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(thread_rng().gen_range(0.0..1.0)),
            )),
            mesh: Mesh2dHandle(BEE_MESH),
            bee_type,
            creature: LivingCreature::from(bee_type),
            rigid_body: RigidBody::from(bee_type),
            behaviour: UniversalBehaviour::from(bee_type),
            gain: CurrencyGainPerMinute::from(bee_type),
            target: NavigationTarget::None,
            result: NavigationResult::default(),
            move_behaviour: MoveToNavigationTargetBehaviour,
            orientation: SmartOrientation,
            faction: Faction::Bees,
        }
    }
}
