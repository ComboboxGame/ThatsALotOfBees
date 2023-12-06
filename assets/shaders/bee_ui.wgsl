#import bevy_ui::ui_vertex_output::UiVertexOutput

struct BeeMaterial {
    phase: f32,
    shape: u32,
    overlay_kind: u32,
    overlay_level: u32,
    flags: u32,
};
const COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

@group(1) @binding(0) var<uniform> material: BeeMaterial;
@group(1) @binding(1) var texture: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

fn mix_colors(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4(a.rgb * (1.0 - b.a) + b.rgb * b.a, a.a * (1.0 - b.a) + b.a);
}

fn get_shape_color(uvi: vec2<f32>) -> vec4<f32> {
    var uv = (uvi + vec2(select(0.0, 4.0, material.shape == 1u), select(0.0, 1.0, material.shape == 2u))) / 8.0;
    return textureSample(texture, texture_sampler, uv);
}

fn get_wing_color(uvi: vec2<f32>) -> vec4<f32> {
    let tick = u32(floor(material.phase * 10.0));
    var uv = (uvi + select(vec2(1.0, 0.0), vec2(2.0, 0.0), tick % 2u == 0u) + select(vec2(0.0, 0.0), vec2(4.0, 0.0), material.shape == 1u)) / 8.0;
    return textureSample(texture, texture_sampler, uv);
}

fn get_overlay_color(uvi: vec2<f32>) -> vec4<f32> {
    var uv = (uvi + vec2(0.0, 2.0 + f32(material.overlay_kind))) / 8.0;
    return textureSample(texture, texture_sampler, uv);
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = vec4(1f, 1f, 1f, 1f);
    if ((material.flags & COLOR_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
        output_color = output_color * get_shape_color(in.uv);
        output_color = mix_colors(output_color, get_wing_color(in.uv));
        output_color = mix_colors(output_color, get_overlay_color(in.uv));
    }
    return output_color;
}