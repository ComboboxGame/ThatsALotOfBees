use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};
use bevy::sprite::{Material2d, Mesh2dPipeline};

use super::BeeKind;

#[derive(AsBindGroup, Debug, Clone, Reflect, Asset)]
#[uniform(0, BeeMaterialUniform)]
pub struct BeeMaterial {
    pub phase: f32,        //
    pub shape: u32,        // 0 - regular, 1 - baby, 2 - queen
    pub overlay_kind: u32, // 0 - regular, 1 - worker, 2 - defender, 3 - builder
    pub overlay_level: u32,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
} 

impl Default for BeeMaterial {
    fn default() -> Self {
        BeeMaterial {
            phase: 0.,
            shape: 0,
            overlay_kind: 0,
            overlay_level: 0,
            texture: None,
        }
    }
}

impl From<BeeKind> for BeeMaterial {
    fn from(kind: BeeKind) -> Self {
        let (shape, overlay_kind, overlay_level) = get_shape_kind_level(kind);
        Self {
            shape,
            overlay_kind,
            overlay_level,
            ..Default::default()
        }
    }
}

impl From<Handle<Image>> for BeeMaterial {
    fn from(texture: Handle<Image>) -> Self {
        BeeMaterial {
            //texture: Some(texture),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct BeeMaterialUniform {
    pub phase: f32,
    pub shape: u32,
    pub overlay_kind: u32,
    pub overlay_level: u32,
    pub flags: u32,
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ColorMaterialFlagsCustom: u32 {
        const TEXTURE           = (1 << 0);
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

impl AsBindGroupShaderType<BeeMaterialUniform> for BeeMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> BeeMaterialUniform {
        let mut flags = ColorMaterialFlagsCustom::NONE;
        if self.texture.is_some() {
            flags |= ColorMaterialFlagsCustom::TEXTURE;
        }

        BeeMaterialUniform {
            phase: self.phase,
            shape: self.shape,
            overlay_kind: self.overlay_kind,
            overlay_level: self.overlay_level,
            flags: flags.bits(),
        }
    }
}

impl Material2d for BeeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\bee.wgsl".into()
    }
}

impl UiMaterial for BeeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\bee_ui.wgsl".into()
    }
}

pub fn get_shape_kind_level(kind: BeeKind) -> (u32, u32, u32) {
    (
        match kind {
            BeeKind::Baby => 1,
            BeeKind::Queen => 2,
            _ => 0,
        },
        match kind {
            BeeKind::Worker => 1,
            BeeKind::Defender => 2,
            BeeKind::Builder => 3,
            _ => 0,
        },
        0
    )
}
