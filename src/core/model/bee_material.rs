use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};
use bevy::sprite::Material2d;
use rand::{thread_rng, Rng};

use super::{BeeKind, WaspKind};

pub const BEE_ATLAS_HANDLE: Handle<Image> = Handle::weak_from_u128(1311196983220122547);
pub const WASP_ATLAS_HANDLE: Handle<Image> = Handle::weak_from_u128(1311192983220225547);

#[derive(Clone, ShaderType, Reflect, Debug)]
pub struct BeeMaterialUniform {
    pub color: Color,
    tiles: u32,
    shape: u32,
    wing_shape: u32,
    overlay_x: u32,
    overlay_y: u32,
    phase: f32,
}

impl Default for BeeMaterialUniform {
    fn default() -> Self {
        BeeMaterialUniform {
            phase: 0.,
            shape: 0,
            tiles: 8,
            wing_shape: 0,
            overlay_x: 7,
            overlay_y: 7,
            color: Color::WHITE,
        }
    }
}

#[derive(AsBindGroup, Debug, Clone, Reflect, Asset, Default)]
#[uniform(0, BeeMaterialUniform)]
pub struct BeeMaterial {
    pub props: BeeMaterialUniform,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl From<BeeKind> for BeeMaterial {
    fn from(kind: BeeKind) -> Self {
        let shape = match kind {
            BeeKind::Baby => 0,
            BeeKind::Queen => 2,
            _ => 1,
        };

        let (overlay_x, overlay_y) = match kind {
            BeeKind::Regular => (0, 2),
            BeeKind::Worker => (0, 3),
            BeeKind::Defender => (0, 4),
            BeeKind::Builder => (0, 5),
            BeeKind::Baby => (7, 7),
            BeeKind::Queen => (7, 7),
        };

        Self {
            props: BeeMaterialUniform {
                color: Color::WHITE,
                tiles: 8,
                shape,
                wing_shape: shape,
                overlay_x,
                overlay_y,
                phase: rand::thread_rng().gen_range(0.0..16.0),
            },
            texture: Some(BEE_ATLAS_HANDLE),
        }
    }
}

impl From<WaspKind> for BeeMaterial {
    fn from(kind: WaspKind) -> Self {
        let shape = match kind {
            WaspKind::Regular => 0,
        };

        let (overlay_x, overlay_y) = match kind {
            WaspKind::Regular => (3, 3),
        };

        BeeMaterial {
            props: BeeMaterialUniform {
                color: Color::WHITE,
                tiles: 4,
                shape,
                wing_shape: shape,
                overlay_x,
                overlay_y,
                phase: rand::thread_rng().gen_range(0.0..16.0),
            },
            texture: Some(WASP_ATLAS_HANDLE),
        }
    }
}

impl AsBindGroupShaderType<BeeMaterialUniform> for BeeMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> BeeMaterialUniform {
        self.props.clone()
    }
}

impl Material2d for BeeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\bee.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders\\vertex\\uvdxdy.wgsl".into()
    }
}

impl UiMaterial for BeeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\bee_ui.wgsl".into()
    }
}

pub fn prepare_atlases_system(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut bee_atlas: Local<Handle<Image>>,
    mut wasp_atlas: Local<Handle<Image>>,
    mut bee_atlas_done: Local<bool>,
    mut wasp_atlas_done: Local<bool>,
) {
    if *wasp_atlas_done && *bee_atlas_done {
        return;
    }

    if *bee_atlas == Handle::default() && !*bee_atlas_done {
        *bee_atlas = asset_server.load("images/Bee.png");
    }
    if *wasp_atlas == Handle::default() && !*wasp_atlas_done {
        *wasp_atlas = asset_server.load("images/Wasp.png");
    }

    let bee_atlas_ready = images.get(bee_atlas.clone()).is_some();
    if bee_atlas_ready && !*bee_atlas_done {
        println!("Bee atlas ready!");
        let bee_atlas_image = images.get(bee_atlas.clone()).unwrap().clone();
        images.insert(BEE_ATLAS_HANDLE, bee_atlas_image);
        *bee_atlas_done = true;
        *bee_atlas = Handle::default();
    }

    let wasp_atlas_ready = images.get(wasp_atlas.clone()).is_some();
    if wasp_atlas_ready && !*wasp_atlas_done {
        println!("Wasp atlas ready!");
        let wasp_atlas_image = images.get(wasp_atlas.clone()).unwrap().clone();
        images.insert(WASP_ATLAS_HANDLE, wasp_atlas_image);
        *wasp_atlas_done = true;
        *wasp_atlas = Handle::default();
    }
}
