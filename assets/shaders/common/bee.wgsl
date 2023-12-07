#define_import_path game::bee_common
#import game::common::mix_colors
struct BeeMaterial {
    color: vec4<f32>,
    tiles: u32,
    shape: u32,
    wing_shape: u32,
    overlay_x: u32,
    overlay_y: u32,
    phase: f32,
    damage_time: f32,
};

const COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

@group(1) @binding(0) var<uniform> material: BeeMaterial;
@group(1) @binding(1) var texture: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

fn get_shape_color(uvi: vec2<f32>) -> vec4<f32> {
    var uv = uvi + vec2(f32(material.shape), 0.0);
    return textureSample(texture, texture_sampler, uv / f32(material.tiles));
}

fn get_wing_color(uvi: vec2<f32>, phase: f32) -> vec4<f32> {
    let tick = u32(floor(phase * 10.0));
    var uv = uvi + vec2(f32(material.wing_shape * 2u + tick % 2u), 1.0);
    return textureSample(texture, texture_sampler, uv / f32(material.tiles));
}

fn get_overlay_color(uvi: vec2<f32>) -> vec4<f32> {
    var uv = uvi + vec2(f32(material.overlay_x), f32(material.overlay_y));
    return textureSample(texture, texture_sampler, uv / f32(material.tiles));
}

fn get_blood_color(uvi: vec2<f32>, intensity: f32) -> vec4<f32> {
    let tick = u32((1.0 - intensity) * 5.0);
    var uv = uvi + vec2(f32(i32(material.tiles) - 1), f32(2u + tick));
    return textureSample(texture, texture_sampler, uv / f32(material.tiles));
}

fn get_color(uv: vec2<f32>, time: f32) -> vec4<f32> {
    var color = get_shape_color(uv);
    color = mix_colors(color, get_overlay_color(uv));
    color = mix_colors(color, get_wing_color(uv, material.phase + time));

    let t = max(time - material.damage_time, 0.0);
    let intensity = max(1.0 - t * 3.0, 0.0);
    let intensity2 = max(1.0 - t * 1.5, 0.0);
    color = vec4(color.xyz * (1.0 - intensity * 0.8), color.w) + intensity * vec4(1.0, 0.0, 0.0, 0.0);
    color = mix_colors(color, get_blood_color(uv, intensity2));

    return color;
}
