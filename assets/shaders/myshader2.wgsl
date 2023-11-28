//!
//! Ported from https://caballerocoll.com/blog/bevy-rhythm-game/
//!
#import bevy_pbr::forward_io::VertexOutput

#import bevy_render::view  View
@group(0) @binding(0) var<uniform> view: View;

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals;

@group(1) @binding(1) var texture: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

fn modulo(c: vec3<f32>, d: f32) -> vec3<f32> {
    return vec3(c.x % d, c.y % d, c.z % d);
}

fn hsb2rgb(c: vec3<f32>) -> vec3<f32> {
    var rgb: vec3<f32> = clamp(abs(modulo(c.x*6.0+vec3(0.0,4.0,2.0),6.0) - 3.0) - 1.0,
                     vec3(0.0),
                     vec3(1.0),
    );
    rgb = rgb*rgb*(3.0 - 2.0 * rgb);
    return c.z * mix( vec3(1.0), rgb, c.y);
}

fn wave_sin(x: f32) -> f32 {
    let amplitude: f32 = 0.5;
    let frequency: f32 = 1.0;
    var y: f32 = sin(x * frequency);
    let t: f32 = 0.01*(-globals.time*50.0);
    y += sin(x * frequency * 2.1 + t)*4.5;
    y += sin(x * frequency * 1.72 + t*1.121)*4.0;
    y += sin(x * frequency * 2.221 + t*0.437)*5.0;
    y += sin(x * frequency * 3.1122+ t*4.269)*2.5;
    y *= amplitude*0.06;
    return y;
}

fn wave_cos(x: f32) -> f32 {
    let amplitude: f32 = 0.5;
    let frequency: f32 = 2.0;
    var y: f32 = cos(x * frequency);
    let t: f32 = 0.01*(-globals.time*30.0);
    y += cos(x * frequency * 2.1 + t)*4.5;
    y += cos(x * frequency * 1.72 + t*1.121)*4.0;
    y += cos(x * frequency * 2.221 + t*0.437)*5.0;
    y += cos(x * frequency * 3.1122+ t*4.269)*2.5;
    y *= amplitude*0.06;
    return y;
}
fn wave(v: vec2<f32>) -> vec2<f32> {
    return vec2(wave_sin(v.x), wave_cos(v.y));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = wave(in.uv);
    let color: vec3<f32> = hsb2rgb(vec3(uv.x + sin(uv.y), 0.7, 1.0));
    // let color: vec3<f32> = lin2srgb(vec3(uv.x + sin(uv.y), 0.7, 1.0));

    return vec4<f32>(color, 1.0);
}
