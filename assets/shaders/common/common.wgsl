#define_import_path game::common

fn mix_colors(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4(a.rgb * (1.0 - b.a) + b.rgb * b.a, a.a * (1.0 - b.a) + b.a);
}

fn average_color(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return (a * sign(a.w) + b * sign(b.w)) / 2.0;
}
