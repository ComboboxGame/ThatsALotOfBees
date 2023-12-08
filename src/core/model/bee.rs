use crate::core::{NavigationTarget, NavigationResult, Faction};

use super::{UniversalMaterial, LivingCreature, RigidBody, UniversalBehaviour, MoveToNavigationTargetBehaviour, SmartOrientation, BEE_MESH};

use bevy::{prelude::*, sprite::Mesh2dHandle};
use rand::{thread_rng, Rng};
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
}

impl From<(BeeType, Vec2)> for BeeBundle {
    fn from(value: (BeeType, Vec2)) -> Self {
        let (bee_type, position) = value;

        BeeBundle {
            visiblity: VisibilityBundle::default(),
            transform: TransformBundle::from_transform(Transform::from_translation(
                position.extend(thread_rng().gen_range(0.0..1.0))
            )),
            mesh: Mesh2dHandle(BEE_MESH),
            bee_type,
            creature: LivingCreature::from(bee_type),
            rigid_body: RigidBody::from(bee_type),
            behaviour: UniversalBehaviour::from(bee_type),
            target: NavigationTarget::None,
            result: NavigationResult::default(),
            move_behaviour: MoveToNavigationTargetBehaviour,
            orientation: SmartOrientation,
            faction: Faction::Bees,
        }
    }
}
