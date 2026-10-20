#![no_std]

pub use crevice;
pub use glam as shared_glam;

use glam::{Vec2, vec2};

#[crevice::std140(pad = scalar, pad_type = u16)]
pub struct Parameter {
    #[crevice(size = 8)]
    pub resolution: Vec2,
    #[crevice(size = 16)]
    pub camera: GpuCamera,
}

#[crevice::std140(pad = scalar, pad_type = u16)]
pub struct GpuCamera {
    #[crevice(size = 8)]
    pub position: Vec2,
    pub zoom: f32,
}

impl GpuCamera {
    pub fn unproject(&self, v: Vec2) -> Vec2 {
        let resized_pos = v / self.zoom;
        resized_pos + vec2(self.position.x, self.position.y)
    }
    pub fn unproject_scalev(&self, v: Vec2) -> Vec2 {
        v / self.zoom
    }
    pub fn unproject_scale(&self, n: f32) -> f32 {
        n / self.zoom
    }
}
