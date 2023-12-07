#import bevy_sprite::{
    mesh2d_view_bindings::{view, globals},
    mesh2d_bindings::mesh,
    mesh2d_functions::{get_model_matrix, mesh2d_position_local_to_clip},
}

#ifdef TONEMAP_IN_SHADER
#import bevy_core_pipeline::tonemapping
#endif

#import bevy_sprite::{
    mesh2d_functions as mesh_functions,
}

#import game::vertex_output::VertexOutput

struct Vertex {
    @builtin(instance_index) instance_index: u32,
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
};


#import bevy_render::{
    instance_index::get_instance_index,
    maths::{affine_to_square, mat2x4_f32_to_mat3x3_unpack},
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {

    var out: VertexOutput;
    var model = mesh_functions::get_model_matrix(vertex.instance_index);

#ifdef VERTEX_UVS
    let a = mesh[get_instance_index(vertex.instance_index)].inverse_transpose_model_a;
    let b = mesh[get_instance_index(vertex.instance_index)].inverse_transpose_model_b;
    let movel_inv = transpose(mat2x2(a[0].xy, vec2(a[0].w, a[1].x)));
    let view_inv = mat2x2(view.inverse_view_proj[0].xy, view.inverse_view_proj[1].xy);

    let w = vertex.position.x / (vertex.uv.x - 0.5);
    let h = vertex.position.y / (vertex.uv.y - 0.5);

    let SCREEN_TO_UV = mat2x2(1.0 / w, 0.0, 0.0, 1.0 / h) * movel_inv * view_inv;

    out.uv = vertex.uv;
    out.uv_dx = SCREEN_TO_UV * vec2(1.0 / view.viewport.z, 0.0);
    out.uv_dy = SCREEN_TO_UV * vec2(0.0, 1.0 / view.viewport.w);
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = mesh_functions::mesh2d_position_local_to_world(
        model,
        vec4<f32>(vertex.position, 1.0)
    );
    out.position = mesh_functions::mesh2d_position_world_to_clip(out.world_position);
#endif

#ifdef VERTEX_NORMALS
    out.world_normal = mesh_functions::mesh2d_normal_local_to_world(vertex.normal, vertex.instance_index);
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh2d_tangent_local_to_world(
        model,
        vertex.tangent
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
    return out;
}
