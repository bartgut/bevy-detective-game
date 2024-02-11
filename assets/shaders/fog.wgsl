#import bevy_pbr::forward_io::VertexOutput
#import bevy_render::globals::Globals

struct FogMaterial {
    fog_size: vec2<f32>,
    fog_color: vec4<f32>,
    fog_speed: vec2<f32>
}

@group(0) @binding(1) var<uniform> globals: Globals;
@group(1) @binding(0) var<uniform> material: FogMaterial;

fn random(uv: vec2<f32>) -> f32 {
    return fract(sin(dot(uv, vec2<f32>(12.9898, 78.233))) * 43758.5453123);
}

// Define a function to generate a simple noise value based on a 2D coordinate
fn noise(uv: vec2<f32>) -> f32 {
    let uv_index: vec2<f32> = floor(uv);
    let uv_fract: vec2<f32> = fract(uv);

    // Four corners in 2D of a tile
    let a: f32 = random(uv_index);
    let b: f32 = random(uv_index + vec2<f32>(1.0, 0.0));
    let c: f32 = random(uv_index + vec2<f32>(0.0, 1.0));
    let d: f32 = random(uv_index + vec2<f32>(1.0, 1.0));

    let blur: vec2<f32> = smoothstep(vec2<f32>(0.0,0.0), vec2<f32>(1.0, 1.0), uv_fract);

    return mix(mix(a, b, blur.x), mix(c, d, blur.x), blur.y);
}

// Define a function to generate a fractal brownian motion value based on a 2D coordinate
fn fbm(uv: vec2<f32>) -> f32 {
    var octaves: i32 = 6;
    var amplitude: f32 = 0.5;
    var frequency: f32 = 3.0;
    var value: f32 = 0.0;

    for (var i: i32 = 0; i < octaves; i = i + 1) {
        value = value + amplitude * noise(uv * frequency);
        amplitude = amplitude * 0.5;
        frequency = frequency * 2.0;
    }
    return value;
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    return material.fog_color*fbm(mesh.uv*material.fog_size + globals.time * material.fog_speed);
}



