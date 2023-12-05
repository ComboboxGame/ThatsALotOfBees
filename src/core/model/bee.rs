use super::{get_shape_kind_level, BeeMaterial};
use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Changed, With},
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
use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, EnumIter)]
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
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    if image_handle.is_none() {
        *image_handle = Some(asset_server.load("images/Bee.png"));
    }

    for (_, material) in materials.iter_mut() {
        material.phase += time.delta_seconds();
    }

    let mut find_material = |kind: BeeKind| -> Handle<BeeMaterial> {
        let (shape, overlay_kind, overlay_level) = get_shape_kind_level(kind);
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
