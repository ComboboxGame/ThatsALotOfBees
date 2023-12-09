#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import game::common::mix_colors

@group(1) @binding(0) var<uniform> progress: vec4<f32>;
@group(1) @binding(1) var<uniform> state: vec4<u32>;
@group(1) @binding(2) var base_color_texture: texture_2d<f32>;
@group(1) @binding(3) var base_color_sampler: sampler;
@group(1) @binding(4) var background_texture: texture_2d<f32>;
@group(1) @binding(5) var background_sampler: sampler;
@group(1) @binding(6) var hovered_texture: texture_2d<f32>;
@group(1) @binding(7) var hovered_sampler: sampler;
@group(1) @binding(8) var selected_texture: texture_2d<f32>;
@group(1) @binding(9) var selected_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let a = textureSample(background_texture, background_sampler, mesh.uv);
    let b = textureSample(base_color_texture, base_color_sampler, mesh.uv);
    let c = textureSample(hovered_texture, hovered_sampler, mesh.uv);
    let d = textureSample(selected_texture, selected_sampler, mesh.uv);

    var color = vec4(0.0);
    if 1.0 - mesh.uv.y < progress.x {
        color = mix_colors(color, a);
    }

    if state.x != 0u {
        if (state.x & 1u) == 1u {
            color = mix_colors(color, c);
        }
        if (state.x & 2u) == 2u {
            color = mix_colors(color, d);
        }
    }

    return mix_colors(color, b);
}
