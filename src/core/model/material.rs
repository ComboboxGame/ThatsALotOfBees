use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};
use bevy::sprite::Material2d;
use rand::Rng;

use super::{BeeType, EnemyType};

pub const BEE_ATLAS_HANDLE: Handle<Image> = Handle::weak_from_u128(1311196983220122547);
pub const WASP_ATLAS_HANDLE: Handle<Image> = Handle::weak_from_u128(1311192983220225545);
pub const BIRB_ATLAS_HANDLE: Handle<Image> = Handle::weak_from_u128(1512192983220215541);
pub const BUMBLE_ATLAS_HANDLE: Handle<Image> = Handle::weak_from_u128(1512192983220115541);

#[derive(Clone, ShaderType, Reflect, Debug)]
pub struct BeeMaterialUniform {
    pub color: Color,
    pub tiles_x: u32,
    pub tiles_y: u32,
    pub wing_states: u32,
    pub shape: u32,
    pub wing_shape: u32,
    pub overlay_x: u32,
    pub overlay_y: u32,
    pub phase: f32,
    pub damage_time: f32,
    pub upgrade_time: f32,
}

impl Default for BeeMaterialUniform {
    fn default() -> Self {
        BeeMaterialUniform {
            phase: 0.,
            damage_time: -1.0,
            upgrade_time: -1.0,
            shape: 0,
            tiles_x: 8,
            tiles_y: 8,
            wing_states: 2,
            wing_shape: 0,
            overlay_x: 7,
            overlay_y: 7,
            color: Color::WHITE,
        }
    }
}

#[derive(AsBindGroup, Debug, Clone, Reflect, Asset, Default)]
#[uniform(0, BeeMaterialUniform)]
pub struct UniversalMaterial {
    pub props: BeeMaterialUniform,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

#[derive(AsBindGroup, Debug, Clone, Reflect, Asset, Default)]
pub struct BuildingMaterial {
    #[uniform(0)]
    pub progress: Vec4,
    #[uniform(1)]
    pub state: UVec4,

    #[texture(2)]
    #[sampler(3)]
    pub texture: Option<Handle<Image>>,

    #[texture(4)]
    #[sampler(5)]
    pub background: Option<Handle<Image>>,

    #[texture(6)]
    #[sampler(7)]
    pub hovered: Option<Handle<Image>>,

    #[texture(8)]
    #[sampler(9)]
    pub selected: Option<Handle<Image>>,
}

impl From<BeeType> for UniversalMaterial {
    fn from(kind: BeeType) -> Self {
        let shape = match kind {
            BeeType::Baby => 0,
            BeeType::Queen => 2,
            _ => 1,
        };

        let (overlay_x, overlay_y) = match kind {
            BeeType::Regular => (0, 2),
            BeeType::Worker(lvl) => (lvl, 3),
            BeeType::Defender(lvl) => (lvl, 4),
            BeeType::Baby => (7, 7),
            BeeType::Queen => (7, 7),
        };

        Self {
            props: BeeMaterialUniform {
                color: Color::WHITE,
                tiles_x: 8,
                tiles_y: 8,
                shape,
                wing_states: 2,
                wing_shape: shape,
                overlay_x,
                overlay_y,
                phase: rand::thread_rng().gen_range(0.0..16.0),
                damage_time: -1.0,
                upgrade_time: -1.0,
            },
            texture: Some(BEE_ATLAS_HANDLE),
        }
    }
}

impl From<EnemyType> for UniversalMaterial {
    fn from(kind: EnemyType) -> Self {
        match kind {
            EnemyType::Wasp(lvl) => UniversalMaterial {
                props: BeeMaterialUniform {
                    color: Color::WHITE,
                    tiles_x: 4,
                    tiles_y: 4,
                    shape: 0,
                    wing_states: 2,
                    wing_shape: 0,
                    overlay_x: [3, 0][lvl as usize],
                    overlay_y: [3, 2][lvl as usize],
                    phase: rand::thread_rng().gen_range(0.0..16.0),
                    damage_time: -1.0,
                    upgrade_time: -1.0,
                },
                texture: Some(WASP_ATLAS_HANDLE),
            },
            EnemyType::Birb(lvl) => UniversalMaterial {
                props: BeeMaterialUniform {
                    color: Color::WHITE,
                    tiles_x: 5,
                    tiles_y: 2,
                    shape: 0,
                    wing_states: 5,
                    wing_shape: 0,
                    overlay_x: [4, 3, 2][lvl as usize],
                    overlay_y: 0,
                    phase: rand::thread_rng().gen_range(0.0..16.0),
                    damage_time: -1.0,
                    upgrade_time: -1.0,
                },
                texture: Some(BIRB_ATLAS_HANDLE),
            },
            EnemyType::Bumble(lvl) => UniversalMaterial {
                props: BeeMaterialUniform {
                    color: Color::WHITE,
                    tiles_x: 4,
                    tiles_y: 2,
                    shape: 0,
                    wing_states: 2,
                    wing_shape: 0,
                    overlay_x: lvl,
                    overlay_y: 0,
                    phase: rand::thread_rng().gen_range(0.0..16.0),
                    damage_time: -1.0,
                    upgrade_time: -1.0,
                },
                texture: Some(BUMBLE_ATLAS_HANDLE),
            },
        }
    }
}

impl AsBindGroupShaderType<BeeMaterialUniform> for UniversalMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> BeeMaterialUniform {
        self.props.clone()
    }
}

impl Material2d for UniversalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\bee.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders\\vertex\\uvdxdy.wgsl".into()
    }
}

impl Material2d for BuildingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\building.wgsl".into()
    }
}

impl UiMaterial for UniversalMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\bee_ui.wgsl".into()
    }
}

pub fn prepare_atlases_system(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,

    mut bee_atlas: Local<Handle<Image>>,
    mut wasp_atlas: Local<Handle<Image>>,
    mut birb_atlas: Local<Handle<Image>>,
    mut bumble_atlas: Local<Handle<Image>>,

    mut bee_atlas_done: Local<bool>,
    mut wasp_atlas_done: Local<bool>,
    mut birb_atlas_done: Local<bool>,
    mut bumble_atlas_done: Local<bool>,
) {
    if *wasp_atlas_done && *bee_atlas_done && *birb_atlas_done && *bumble_atlas_done {
        return;
    }

    if *bee_atlas == Handle::default() && !*bee_atlas_done {
        *bee_atlas = asset_server.load("images/Bee.png");
    }
    if *wasp_atlas == Handle::default() && !*wasp_atlas_done {
        *wasp_atlas = asset_server.load("images/Wasp.png");
    }
    if *birb_atlas == Handle::default() && !*birb_atlas_done {
        *birb_atlas = asset_server.load("images/Birb.png");
    }
    if *bumble_atlas == Handle::default() && !*bumble_atlas_done {
        *bumble_atlas = asset_server.load("images/Bumble.png");
    }

    let bee_atlas_ready = images.get(bee_atlas.clone()).is_some();
    if bee_atlas_ready && !*bee_atlas_done {
        let bee_atlas_image = images.get(bee_atlas.clone()).unwrap().clone();
        images.insert(BEE_ATLAS_HANDLE, bee_atlas_image);
        *bee_atlas_done = true;
        *bee_atlas = Handle::default();
    }

    let wasp_atlas_ready = images.get(wasp_atlas.clone()).is_some();
    if wasp_atlas_ready && !*wasp_atlas_done {
        let wasp_atlas_image = images.get(wasp_atlas.clone()).unwrap().clone();
        images.insert(WASP_ATLAS_HANDLE, wasp_atlas_image);
        *wasp_atlas_done = true;
        *wasp_atlas = Handle::default();
    }

    let birb_atlas_ready = images.get(birb_atlas.clone()).is_some();
    if birb_atlas_ready && !*birb_atlas_done {
        let birb_atlas_image = images.get(birb_atlas.clone()).unwrap().clone();
        images.insert(BIRB_ATLAS_HANDLE, birb_atlas_image);
        *birb_atlas_done = true;
        *birb_atlas = Handle::default();
    }

    let bumble_atlas_ready = images.get(bumble_atlas.clone()).is_some();
    if bumble_atlas_ready && !*bumble_atlas_done {
        let bumble_atlas_image = images.get(bumble_atlas.clone()).unwrap().clone();
        images.insert(BUMBLE_ATLAS_HANDLE, bumble_atlas_image);
        *bumble_atlas_done = true;
        *bumble_atlas = Handle::default();
    }
}
