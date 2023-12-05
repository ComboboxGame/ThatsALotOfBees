use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::Changed,
        system::{Commands, Local, Query, Res, ResMut},
    },
    input::{mouse::MouseButton, Input},
    math::Vec2,
    render::texture::Image,
    time::Time,
    transform::components::Transform,
    utils::{Entry, HashMap},
    window::CursorMoved,
};
use rand::{thread_rng, Rng};

use super::BeeMaterial;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
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
pub struct Bee {
    pub kind: BeeKind,
    pub target: Vec2,
}

pub fn update_bee_material_system(
    mut commands: Commands,
    mut bees: Query<(Entity, &Bee, Option<&mut Handle<BeeMaterial>>), Changed<Bee>>,
    mut materials: ResMut<Assets<BeeMaterial>>,
    mut materials_table: Local<HashMap<u32, Handle<BeeMaterial>>>,
    mut image_handle: Local<Option<Handle<Image>>>,
    mut asset_server: ResMut<AssetServer>,
    mut time_bank: Local<f32>,
    time: Res<Time>,
) {
    if image_handle.is_none() {
        *image_handle = Some(asset_server.load("images/Bee.png"));
    }

    for (_, material) in materials.iter_mut() {
        material.phase += time.delta_seconds();
    }

    let mut find_material = |kind: BeeKind| -> Handle<BeeMaterial> {
        let shape = match kind {
            BeeKind::Baby => 1,
            BeeKind::Queen => 2,
            _ => 0,
        };

        let overlay_kind = match kind {
            BeeKind::Worker => 1,
            BeeKind::Defender => 2,
            BeeKind::Builder => 3,
            _ => 0,
        };

        let overlay_level = 0;

        let key =
            shape + overlay_kind * 8 + overlay_level * 64 + thread_rng().gen_range(0..4) * 2048;
        match materials_table.entry(key) {
            Entry::Occupied(handle) => handle.get().clone(),
            Entry::Vacant(vacant) => vacant
                .insert(materials.add(BeeMaterial {
                    phase: thread_rng().gen_range(0.0..16.0),
                    shape,
                    overlay_kind,
                    overlay_level,
                    texture: image_handle.clone(),
                }))
                .clone(),
        }
        /*materials.add(BeeMaterial {
            phase: thread_rng().gen_range(0.0..16.0),
            shape,
            overlay_kind,
            overlay_level,
            texture: image_handle.clone(),
        })*/
    };

    for (e, bee, maybe_material) in bees.iter_mut() {
        if let Some(mut material) = maybe_material {
            let m = find_material(bee.kind);
            if *material != m {
                *material = m;
            }
        } else {
            commands.entity(e).insert(find_material(bee.kind));
        }
    }
}
