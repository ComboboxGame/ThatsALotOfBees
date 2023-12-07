#import bevy_ui::ui_vertex_output::UiVertexOutput

#import game::common::{mix_colors}
#import game::bee_common::{BeeMaterial, COLOR_MATERIAL_FLAGS_TEXTURE_BIT, get_color, material}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = vec4(1f, 1f, 1f, 1f);
    output_color = output_color * get_color(in.uv, 0.0);
    return output_color;
}
