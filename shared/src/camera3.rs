use glam::{Vec2, Vec3, vec3};

#[allow(unused_imports)] // required by methods like 10.0.powi(...)
use spirv_std::num_traits::*;

#[crevice::std140(pad = scalar, pad_type = u16)]
pub struct Viewport3 {
    #[crevice(size = 8)]
    pub resolution: Vec2,
    #[crevice(size = 32)]
    pub camera: Camera3,
}

#[crevice::std140(pad = scalar, pad_type = u16)]
#[derive(Clone)]
pub struct Camera3 {
    #[crevice(size = 12)]
    pub position: Vec3,
    pub pitch: f32,
    pub yaw: f32,
    pub pov: f32,
}

impl Camera3 {
    pub fn world_to_view(&self, pos: Vec3, with_position: bool) -> Vec3 {
        (pos - if with_position {
            self.position
        } else {
            Vec3::ZERO
        })
        .rotate_y(-self.yaw)
        .rotate_x(-self.pitch)
    }
    pub fn view_to_world(&self, pos: Vec3, with_position: bool) -> Vec3 {
        pos.rotate_x(self.pitch).rotate_y(self.yaw)
            + if with_position {
                self.position
            } else {
                Vec3::ZERO
            }
    }

    pub fn view_to_clip(&self, pos: Vec3) -> Vec3 {
        if pos.z == 0.0 {
            Vec3::ZERO
        } else {
            let plane_radius = self.pov.tan() * pos.z;
            vec3(pos.x / plane_radius, pos.y / plane_radius, pos.z)
        }
    }
    pub fn clip_to_view(&self, pos: Vec3) -> Vec3 {
        let plane_radius = self.pov.tan() * pos.z;
        vec3(pos.x * plane_radius, pos.y * plane_radius, pos.z)
    }

    pub fn world_to_clip(&self, pos: Vec3, with_position: bool) -> Vec3 {
        self.view_to_clip(self.world_to_view(pos, with_position))
    }
    pub fn clip_to_world(&self, pos: Vec3, with_position: bool) -> Vec3 {
        self.view_to_world(self.clip_to_view(pos), with_position)
    }
}
