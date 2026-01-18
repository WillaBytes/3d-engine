struct Camera {
    position: vec3<f32>,
    angle_horizontal: f32,
    angle_vertical: f32,
    scale_factor: f32,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) camera_position: vec3<f32>,
    @location(3) camera_matrix_row_1: vec3<f32>,
    @location(4) camera_matrix_row_2: vec3<f32>,
    @location(5) camera_matrix_row_3: vec3<f32>,
	@location(6) depth_factor: f32,
    @location(7) normal: vec3<f32>,
    @location(8) light_source: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    var render_distance: f32 = 1000.0;

	var camera_matrix = mat3x3<f32>(
		model.camera_matrix_row_1,
		model.camera_matrix_row_2,
		model.camera_matrix_row_3,
	);

    var result = camera_matrix * (model.position - model.camera_position);

    var factor: f32 = 1.0;

    // Attempt 1: Doesn't transform vertices behind camera correctly, and results in vertices parallell to the camera wandering off to infinity.
    /*
    factor = 1.0 / (model.depth_factor * result.z);
    */

    // Attempt 2: Works poorly but better than nothing.
    /*
    if result.z > 0.0 || result.z < -1.0 {
        factor = 1.0 / (model.depth_factor * result.z);
    } else {
        factor = 100.0;
    }
    */

    // Attempt 2: Better, but lacks perspective if z -> 0, because it looks like things are 1 unit of length away. Should an object at length 0 even be infinitely large? Feels intuitive.
    /*
    if result.z >= 0.0 {
        factor = 1.0 / (model.depth_factor * (result.z + 1.0));
    } else {
        factor = 1.0 / model.depth_factor * -(result.z - 1.0);
    }
    */

    // Attempt 3: Hopefully the solution; inshallah.
    /*
    var offset: f32 = 3.0;
    var depth_factor: f32 = 1.0;
    factor = 1.0 / (result.z + 0.1);
    */
    // It didn't work; outshallah.

    // Attempt 4: Slightly better than attempt 1, and best yet. If all vertices are in front of the camera: things are normal; if they are behind the camera: things get quirky at night. At least they don't switch sides this time around.
    /*
    factor = 1.0 / (model.depth_factor * abs(result.z));
    */

    // Attempt 5: Nope, definitely not.
    /*
    factor = 1.0 / (2.0 * abs(result.z) + 1.0) + 1.0;
    */

    // Attempt 6: Starting to think perspective isn't that simple.
    /*
    var scale_factor: f32 = 300.0;
    factor = 1.0 / (result.z + 1.0);
    */

    // Attempt 7: Antics if vertex far away is behind camera, otherwise decent.
    /*
    var limit: f32 = 0.001;
    if result.z > limit {
        factor = 1.0 / result.z;
    } else {
        factor = abs(result.z) * abs(result.z) + 1.0 / limit;
    }
    */

    // Attempt 8: Wonky in the same way as attempt 7, but a little less so.
    var limit: f32 = 0.00001;
    if result.z > limit {
        factor = 1.0 / result.z;
    } else if result.z < -limit {
        factor = -result.z + 1.0 / limit;
    } else {
        factor = 1.0 / limit;
    }

    result.x *= factor;
    result.y *= factor;
    result.z /= render_distance;

	out.clip_position = vec4<f32>(result, 1.0);

    // With lighting
    /*
    var lighting_factor = dot(model.normal, normalize(model.position - model.light_source));
    if lighting_factor < 0.2 { lighting_factor = 0.2; }
    out.color = model.color * lighting_factor;
    */

    // Without lighting
    out.color = model.color;

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
