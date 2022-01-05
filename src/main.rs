use raytrace::vec3::{write_color, Vec3};
use std::io::Write;

fn main() {
    let width = 256;
    let height = 256;

    let mut img = std::fs::File::create("test.ppm").expect("Failed to create image");

    img.write_fmt(format_args!("P3\n{} {}\n255\n", width, height))
        .expect("write failed");

    for j in (0..height).rev() {
        // println!("\rRendering: {}/{}", j, height);
        for i in 0..width {
            let rgb = Vec3 {
                x: i as f64 / (width as f64 - 1.0),
                y: j as f64 / (height as f64 - 1.0),
                z: 0.25,
            };

            write_color(&img, rgb);
        }
    }
}
