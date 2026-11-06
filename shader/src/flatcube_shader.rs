use shared::{
    camera3::{self, Camera3, Viewport3},
    crevice::std140::AsStd140,
};
use spirv_std::{
    glam::{Vec2, Vec3, Vec4, camera, vec2, vec3, vec4},
    spirv,
};

#[allow(unused_imports)] // required by methods like 10.0.powi(...)
use spirv_std::num_traits::*;

type StdViewport3 = <Viewport3 as AsStd140>::Output;

const CENTER: Vec3 = vec3(0.0, 0.0, 10.0);
const RADIUS: f32 = 1.0;

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] viewport: &StdViewport3,
    output: &mut Vec4,
) {
    let frag_coord = vec2(frag_coord.x, frag_coord.y);
    let viewport = Viewport3::from_std140(viewport.clone());
    let screen_radius = if (viewport.resolution.x < viewport.resolution.y) {
        viewport.resolution.x / 2.0
    } else {
        viewport.resolution.y / 2.0
    };

    let clipspace_frag_coord =
        (frag_coord - viewport.resolution / 2.0) * vec2(1.0, -1.0) / screen_radius;

    let marching_direction = viewport
        .camera
        .view_to_world(
            viewport
                .camera
                .clip_to_view(vec3(clipspace_frag_coord.x, clipspace_frag_coord.y, 1.0)),
            false,
        )
        .normalize();
    let mut current_position = viewport.camera.position;
    let mut min_distance = f32::MAX;
    let mut iterations = 0;

    loop {
        // let distance = CENTER.distance(current_position) - 10.0;
        let distance = mandelbulb(current_position);

        if distance < min_distance {
            min_distance = distance;
        }

        if distance < 0.0001 {
            let brightness = 1.0 / (iterations as f32 + 1.0);
            let brightness = brightness * 4.0;
            *output = vec4(brightness, 0., brightness, 0.);
            return;
        } else {
            current_position += marching_direction * distance;
        }

        if 200 <= iterations {
            break;
        }
        iterations += 1;
    }

    if min_distance < 0.1 {
        min_distance = 0.1;
    }

    *output = vec4(0., 0., 0., 0.);
}

fn mandelbulb(pos: Vec3) -> f32 {
    let mut z = pos;
    let mut dr = 1.0f32;
    let mut r = 0.0f32;
    let mut iterations = 0;
    let power = 8.0f32;

    for i in 0..50 {
        iterations = i;
        r = z.length();
        if (r > 2.0) {
            break;
        }

        let mut theta = (z.z / r).acos();
        let mut phi = z.y.atan2(z.x);

        dr = r.pow(power - 1.0) * power * dr + 1.0;

        let zr = r.pow(power);
        theta = theta * power;
        phi = phi * power;

        z = zr
            * vec3(
                theta.sin() * phi.cos(),
                phi.sin() * theta.sin(),
                theta.cos(),
            );
        z += pos;
    }

    return 0.5 * r.log(10.0) * r / dr;
}
