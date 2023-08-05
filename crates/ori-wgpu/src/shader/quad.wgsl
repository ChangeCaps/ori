struct Uniforms {
	resolution: vec2<f32>,
	top_left: vec2<f32>,
	bottom_right: vec2<f32>,
	color: vec4<f32>,
	border_color: vec4<f32>,
	border_radius: vec4<f32>,
	border_width: vec4<f32>,
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
}

struct VertexOutput {
	@builtin(position) clip: vec4<f32>,
	@location(0) uv: vec2<f32>,
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	let position = in.position / uniforms.resolution * vec2<f32>(2.0, -2.0) - vec2<f32>(1.0, -1.0);
	out.clip = vec4<f32>(position, 0.0, 1.0);
	out.uv = in.uv;

	return out;
}

fn quad_distance(
	position: vec2<f32>,
	top_left: vec2<f32>,
	bottom_right: vec2<f32>,
	radius: f32,
) -> f32 {
	let top_left_distance = top_left - position + radius;
	let bottom_right_distance = position - bottom_right + radius;

	let dist = vec2<f32>(
		max(max(top_left_distance.x, bottom_right_distance.x), 0.0),
		max(max(top_left_distance.y, bottom_right_distance.y), 0.0),
	);

	return length(dist);
}

fn select_border_radius(
	position: vec2<f32>, 
	top_left: vec2<f32>, 
	bottom_right: vec2<f32>,
	radi: vec4<f32>,
) -> f32 {
	let center = (top_left + bottom_right) / 2.0;

	let rx = select(radi.x, radi.y, position.x > center.x);
	let ry = select(radi.w, radi.z, position.x > center.x);
	return select(rx, ry, position.y > center.y);
}

fn select_border_width(
	position: vec2<f32>, 
	top_left: vec2<f32>, 
	bottom_right: vec2<f32>,
	width: vec4<f32>,
	radius: f32,
) -> f32 {
	let center = (top_left + bottom_right) / 2.0;
	let diff = position - center;
	var dx = select(
		position.x - top_left.x - max(width.w, radius), 
		bottom_right.x - position.x - max(width.y, radius),
		diff.x > 0.0
	);
	var dy = select(
		position.y - top_left.y - max(width.x, radius),
		bottom_right.y - position.y - max(width.z, radius),
		diff.y > 0.0
	);

	let wx = select(width.w, width.y, diff.x > 0.0);
	let wy = select(width.x, width.z, diff.y > 0.0);
	return max(select(0.0, wx, dx < 0.0), select(0.0, wy, dy < 0.0));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
	let image_color = textureSample(image_texture, image_sampler, in.uv);

	var color = uniforms.color * image_color;

	let border_radius = select_border_radius(
		in.clip.xy,
		uniforms.top_left,
		uniforms.bottom_right,
		uniforms.border_radius,
	);

	let border_width = select_border_width(
		in.clip.xy,
		uniforms.top_left,
		uniforms.bottom_right,
		uniforms.border_width,
		border_radius,
	);

	if border_width > 0.0 {
		let internal_border = max(border_radius - border_width, 0.0);

		let internal_dist = quad_distance(
			in.clip.xy,
			uniforms.top_left + vec2<f32>(border_width),
			uniforms.bottom_right - vec2<f32>(border_width),
			internal_border,
		);

		let border_mix = smoothstep(
			max(internal_border - 0.5, 0.0),
			internal_border + 0.5,
			internal_dist,
		);

		color = mix(color, uniforms.border_color, border_mix);
	}

	let dist = quad_distance(
		in.clip.xy,
		uniforms.top_left,
		uniforms.bottom_right,
		border_radius,
	);

	let radius_alpha = 1.0 - smoothstep(
		max(border_radius - 0.5, 0.0),
		border_radius + 0.5,
		dist,
	);

	return vec4<f32>(
		color.x,
		color.y,
		color.z,
		color.w * radius_alpha,
	);
}
