use glam::Vec2;

#[crevice::std140(pad = scalar, pad_type = u16)]
pub struct Viewport2 {
    #[crevice(size = 8)]
    pub resolution: Vec2,
    #[crevice(size = 16)]
    pub camera: Camera2,
}

#[crevice::std140(pad = scalar, pad_type = u16)]
pub struct Camera2 {
    #[crevice(size = 8)]
    pub position: Vec2,
    pub zoom: f32,
}

impl Camera2 {
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
