use raytrace::ray::Ray;
use raytrace::vec3::{write_color, Vec3};
use std::io::Write;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = r.direction.dot(&r.direction);
    let b = 2.0 * oc.dot(&r.direction);
    let c = oc.dot(&oc) - radius * radius;
    b * b - 4.0 * a * c > 0.0
}

fn ray_color(r: &Ray) -> Vec3 {
    if !hit_sphere(
        &Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        2.5,
        r,
    ) {
        return Vec3 {
            x: 1.0 / 45.0,
            y: 1.0 / 117.0,
            z: 1.0 / 4.0,
        };
    }

    println!("{:?}", r.direction);
    let unit_dir = r.direction.unit();
    let t = 0.5 * (unit_dir.y + 1.0);
    Vec3::ONES * (1.0 - t)
        + Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t
}

fn main() {
    let ratio = 16.0 / 9.0;
    let width = 900 as u32;
    let height = (width as f32 / ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = ratio * viewport_height;
    let focal_len = 1.0;

    let orig = Vec3::ZEROES;
    let horiz = Vec3 {
        x: viewport_width as f64,
        y: 0.0,
        z: 0.0,
    };
    let vert = Vec3 {
        x: 0.0,
        y: viewport_height as f64,
        z: 0.0,
    };

    let lower_left_corner = orig
        - horiz / 2.0
        - vert / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_len,
        };

    let mut img = std::fs::File::create("test.ppm").expect("Failed to create image");

    img.write_fmt(format_args!("P3\n{} {}\n255\n", width, height))
        .expect("write failed");

    for j in (0..height).rev() {
        // println!("\rRendering: {}/{}", j, height);
        for i in 0..width {
            let u = (i as f32 / (width as f32 - 1.0)) as f64;
            let v = (j as f32 / (height as f32 - 1.0)) as f64;
            let r = Ray {
                origin: orig,
                direction: lower_left_corner + horiz * u + vert * v + orig,
            };

            let pixel_color = ray_color(&r);

            write_color(&img, pixel_color);
        }
    }
}
