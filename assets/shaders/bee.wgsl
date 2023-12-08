#import bevy_sprite::{mesh2d_view_bindings::{view, globals}, mesh2d_functions as mesh_functions}
#import game::common::average_color
#import game::vertex_output::VertexOutput
#import game::bee_common::{COLOR_MATERIAL_FLAGS_TEXTURE_BIT, get_color, material}

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = vec4(1f, 1f, 1f, 1f);
    
#ifdef VERTEX_COLORS
    output_color = output_color * mesh.color;
#endif

    let step = 0.3;

        let uv0 = mesh.uv + mesh.uv_dx * step + mesh.uv_dy * step;
        let uv1 = mesh.uv - mesh.uv_dx * step + mesh.uv_dy * step;
        let uv2 = mesh.uv - mesh.uv_dx * step - mesh.uv_dy * step;
        let uv3 = mesh.uv + mesh.uv_dx * step - mesh.uv_dy * step;

        output_color = output_color * 
            average_color(
                average_color(
                    get_color(uv0, globals.time), 
                    get_color(uv1, globals.time)), 
                average_color(
                    get_color(uv2, globals.time),
                    get_color(uv3, globals.time))
            );

#ifdef TONEMAP_IN_SHADER
    output_color = tonemapping::tone_mapping(output_color, view.color_grading);
#endif

    return output_color;
}
