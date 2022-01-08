use crate::vec3::Vec3;
use rand::prelude::*;
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

pub fn rand_vec3_in_sphere() -> Vec3 {
    let mut p;
    loop {
        p = Vec3::rand(-1.0, 1.0);
        if p.len_sqr() >= 1.0 {
            continue;
        } else {
            break;
        }
    }
    p
}

pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_unit_vec3() -> Vec3 {
    rand_vec3_in_sphere().unit()
}

pub fn write_color(mut file: &fs::File, color: Vec3, samples: u64) {
    let scale = 1.0 / samples as f64;
    let r = (256.0 * clamp((color.x * scale).sqrt(), 0.0, 0.999)) as u8;
    let g = (256.0 * clamp((color.y * scale).sqrt(), 0.0, 0.999)) as u8;
    let b = (256.0 * clamp((color.z * scale).sqrt(), 0.0, 0.999)) as u8;

    file.write_fmt(format_args!("{} {} {}\n", r, g, b))
        .expect("Write failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_within_bounds() {
        let mut x = 20.0;
        let min = 5.0;
        let max = 10.0;
        assert_eq!(clamp(x, min, max), 10.0);

        x = 7.0;
        assert_eq!(clamp(x, min, max), 7.0);

        x = 1.0;
        assert_eq!(clamp(x, min, max), 5.0);
    }
}
