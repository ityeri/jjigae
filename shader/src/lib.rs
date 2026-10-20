#![no_std]

use core::f32::consts::{PI, TAU};

use num_complex::{Complex32, ComplexFloat};
use shared::Parameter;
use shared::crevice::std140::AsStd140;
use shared::shared_glam::Vec2;
use spirv_std::glam::{Vec4, vec2, vec4};
use spirv_std::spirv;

pub type StdParameter = <Parameter as AsStd140>::Output;

#[spirv(vertex)]
pub fn main_vs(#[spirv(vertex_index)] vert_id: u32, #[spirv(position)] out_pos: &mut Vec4) {
    let x = ((vert_id << 1) & 2) as f32 * 2.0 - 1.0;
    let y = (vert_id & 2) as f32 * 2.0 - 1.0;
    *out_pos = vec4(x, y, 0.0, 1.0);
}

#[spirv(fragment)]
pub fn main_fs(
    #[spirv(frag_coord)] frag_coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] param: &StdParameter,
    output: &mut Vec4,
) {
    let param = Parameter::from_std140(param.clone());

    let subpixels = 2;
    let frag_coord = vec2(frag_coord.x, frag_coord.y);

    let mut final_color = Vec4::ZERO;

    for sub_y in 0..subpixels {
        for sub_x in 0..subpixels {
            let screen_coord = vec2(
                frag_coord.x + 1.0 / subpixels as f32 * sub_x as f32,
                frag_coord.y + 1.0 / subpixels as f32 * sub_y as f32,
            );
            final_color += calculate_color(to_world_coord(screen_coord, &param))
                / (subpixels * subpixels) as f32;
        }
    }

    *output = final_color

    // *output = (0..subpixels)
    //     .flat_map(|sub_y| {
    //         (0..subpixels).map(move |sub_x| {
    //             vec2(
    //                 frag_coord.x + 1.0 / subpixels as f32 * sub_x as f32,
    //                 frag_coord.y + 1.0 / subpixels as f32 * sub_y as f32,
    //             )
    //         })
    //     })
    //     .map(|screen_coord| calculate_color(to_world_coord(screen_coord, params)))
    //     .reduce(|a, b| a + b)
    //     .unwrap()
    //     / (subpixels * subpixels) as f32;

    // let world_coord = to_world_coord(frag_coord, params);

    // *output = calculate_color(world_coord);
}

fn to_world_coord(pos: Vec2, param: &Parameter) -> Vec2 {
    let camera_coord = {
        let screen_radius = if param.resolution.x > param.resolution.y {
            param.resolution.x / 2.0
        } else {
            param.resolution.y / 2.0
        };
        (pos - vec2(param.resolution.x / 2.0, param.resolution.y / 2.0)) / screen_radius
            * vec2(1.0, -1.0)
    };

    param.camera.unproject(camera_coord)
}

fn calculate_color(pos: Vec2) -> Vec4 {
    let mut z = Complex32::new(pos.x, pos.y);
    let c = z;
    let mut iterations = 0;

    loop {
        // z = z * z + c;
        z = (z * z * z) / (1.0 + Complex32::new(z.re.powi(2), -z.im.powi(2))) + c;
        iterations += 1;

        if 50 <= iterations {
            break;
        }
        if 20.0 < (z.re.powi(2) + z.im.powi(2)).sqrt() {
            break;
        }
    }

    if (z.re.powi(2) + z.im.powi(2)).sqrt() < 20.0 {
        vec4(0., 0., 0., 0.)
    } else {
        cyan_pink_color((iterations % 100) as f32 / 100.0)
    }
}

pub fn cyan_pink_color(v: f32) -> Vec4 {
    vec4((v * TAU).sin(), (v * TAU).cos(), 1.0, 0.)
}

pub fn main() {}
