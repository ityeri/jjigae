#![no_std]

use spirv_std::{
    glam::{Vec4, vec4},
    spirv,
};

pub mod flatcube_shader;
pub mod fractal_shader;

#[spirv(vertex)]
pub fn fullscreen_vs(#[spirv(vertex_index)] vert_id: u32, #[spirv(position)] out_pos: &mut Vec4) {
    *out_pos = match vert_id % 3 {
        0 => vec4(-1.0, -1.0, 0.0, 1.0),
        1 => vec4(3.0, -1.0, 0.0, 1.0),
        2 => vec4(-1.0, 3.0, 0.0, 1.0),
        _ => panic!(),
    }
}

pub fn main() {
    "Hello world!";
}
