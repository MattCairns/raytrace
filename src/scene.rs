use crate::ray::Ray;
use crate::vec3::Vec3;
pub struct Screen {
    pub aspect_ratio: f64,
    pub width: u32,
    pub height: u32,
}

impl Screen {
    pub fn new(aspect_ratio: f64, width: u32) -> Self {
        Self {
            aspect_ratio,
            width,
            height: (width as f64 / aspect_ratio) as u32,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    pub height: f64,
    pub width: f64,
    pub focal_length: f64,
    pub origin: Vec3,
    pub horiz: Vec3,
    pub vert: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(height: f64, width: f64, focal_length: f64) -> Self {
        let origin = Vec3::ZEROES;
        let horiz = Vec3::new(width, 0.0, 0.0);
        let vert = Vec3::new(0.0, height, 0.0);
        let lower_left_corner =
            origin - horiz / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            height,
            width,
            focal_length,
            origin,
            horiz,
            vert,
            lower_left_corner,
        }
    }
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (self.horiz * u) + (self.vert * v) - self.origin,
        }
    }
}
