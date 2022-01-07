use crate::vec3::Vec3;
use std::{fs, io::Write};
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub fn write_color(mut file: &fs::File, color: Vec3, samples: u64) {
    let scale = 1.0 / samples as f64;
    let r = (256.0 * clamp((color.x * scale).sqrt(), 0.0, 0.999)) as u8;
    let g = (256.0 * clamp((color.y * scale).sqrt(), 0.0, 0.999)) as u8;
    let b = (256.0 * clamp((color.z * scale).sqrt(), 0.0, 0.999)) as u8;

    file.write_fmt(format_args!("{} {} {}\n", r, g, b))
        .expect("Write failed");
}
