#define_import_path game::bee_common
#import game::common::mix_colors
struct BeeMaterial {
    color: vec4<f32>,
    tiles_x: u32,
    tiles_y: u32,
    wing_states: u32,
    shape: u32,
    wing_shape: u32,
    overlay_x: u32,
    overlay_y: u32,
    phase: f32,
    damage_time: f32,
    upgrade_time: f32,
};

const COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

@group(1) @binding(0) var<uniform> material: BeeMaterial;
@group(1) @binding(1) var texture: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

fn get_shape_color(uvi: vec2<f32>) -> vec4<f32> {
    var uv = uvi + vec2(f32(material.shape), 0.0);
    return textureSample(texture, texture_sampler, uv / vec2(f32(material.tiles_x), f32(material.tiles_y)));
}

fn get_wing_color(uvi: vec2<f32>, phase: f32) -> vec4<f32> {
    let tick = u32(floor(phase * 10.0));
    var uv = uvi + vec2(f32(material.wing_shape * material.wing_states + tick % material.wing_states), 1.0);
    return textureSample(texture, texture_sampler, uv / vec2(f32(material.tiles_x), f32(material.tiles_y)));
}

fn get_overlay_color(uvi: vec2<f32>) -> vec4<f32> {
    var uv = uvi + vec2(f32(material.overlay_x), f32(material.overlay_y));
    return textureSample(texture, texture_sampler, uv / vec2(f32(material.tiles_x), f32(material.tiles_y)));
}

fn get_blood_color(uvi: vec2<f32>, intensity: f32) -> vec4<f32> {
    let tick = u32((1.0 - intensity) * 5.0);
    var uv = uvi + vec2(f32(i32(material.tiles_x) - 1), f32(2u + tick));
    return textureSample(texture, texture_sampler, uv / vec2(f32(material.tiles_x), f32(material.tiles_y)));
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

    /*var upgrade_radius = max(time - material.upgrade_time, 0.0);
    upgrade_radius = step(upgrade_radius, 0.99) * upgrade_radius;

    let v = (uv - 0.5) * 2.0;

    let d = v.x * v.x + v.y * v.y;
    if (d < upgrade_radius * upgrade_radius  && d > (upgrade_radius - 0.08) * (upgrade_radius - 0.08)) {
        color = mix_colors(vec4(1.0), color);
    }*/

    var upgrade_intensity = max(time - material.upgrade_time, 0.0);
    upgrade_intensity = max(1.0 - upgrade_intensity * 2.0, 0.0);
    color = color * vec4(vec3(10.0, 10.0, 10.0) * upgrade_intensity + vec3(1.0), 1.0);

    return color;
}
