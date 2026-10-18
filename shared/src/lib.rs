#![no_std]

use crevice::std140::AsStd140;

pub use crevice;
pub use glam as shared_glam;

use glam::Vec2;

#[derive(AsStd140)]
pub struct Parameter {
    pub resolution: Vec2,
    pub camera: GpuCamera,
}

#[derive(AsStd140)]
pub struct GpuCamera {
    pub position: Vec2,
    pub zoom: f32,
}

impl GpuCamera {
    pub fn unproject(&self, v: Vec2) -> Vec2 {
        let resized_pos = v / self.zoom;
        resized_pos + self.position
    }
    pub fn unproject_scalev(&self, v: Vec2) -> Vec2 {
        v / self.zoom
    }
    pub fn unproject_scale(&self, n: f32) -> f32 {
        n / self.zoom
    }
}
