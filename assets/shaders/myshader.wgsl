//!
//! A shader.
//!
#import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_view_bindings::globals;
#import bevy_pbr::utils PI
#import shadplay::shader_utils::common NEG_HALF_PI, shader_toy_default, rotate2D

#import bevy_render::view  View
@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(1) var texture: texture_2d<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

const SPEED:f32 = 1.0;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var tiled_uv: vec2<f32>;
    var tiled_uv_x: f32;
    var tiled_uv_y: f32;
    tiled_uv_x = fract(in.uv.x * 4.0);
    tiled_uv_y = fract(in.uv.y * 3.0);
    tiled_uv = vec2(tiled_uv_x,tiled_uv_y);
    return textureSample(texture, texture_sampler, tiled_uv);
}


fn circle(p: vec2<f32>, r: f32) -> f32 {
	return smoothstep(0.1, 0., abs(length(p) - r));
}
