// 'uv's are in the MeshVertexOutput
#import bevy_pbr::forward_io::VertexOutput

@fragment
fn fragment(in: VertexOutput) -> vec4<f32> {
	// usually you'll see them used in your fragment shaders thusly:
	let uv = in.uv;
    let normalised_uv = (in.uv.xy * 2.0) - 1.0; // If you want 0,0 to be at the 'center' of your Mesh's geometry.

	return vec4f(uv.x, uv.y, 0.0, 1.0);
}
