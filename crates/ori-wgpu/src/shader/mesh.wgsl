struct Uniforms {
	resolution: vec2<f32>,	
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var image_texture: texture_2d<f32>;

@group(1) @binding(1)
var image_sampler: sampler;

struct VertexInput {
	@location(0) position: vec2<f32>,
	@location(1) uv: vec2<f32>,
	@location(2) color: vec4<f32>,
}

struct VertexOutput {
	@builtin(position) position: vec4<f32>,
	@location(0) uv: vec2<f32>,
	@location(1) color: vec4<f32>,
}

fn screen_to_clip(position: vec2<f32>) -> vec2<f32> {
	return position / uniforms.resolution * vec2<f32>(2.0, -2.0) - vec2<f32>(1.0, -1.0);
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	out.position = vec4<f32>(screen_to_clip(in.position), 0.0, 1.0);
	out.uv = in.uv;
	out.color = in.color;

	return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
	return in.color * textureSample(image_texture, image_sampler, in.uv);
}
