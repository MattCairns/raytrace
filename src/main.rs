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
            let r = i as f64 / (width as f64 - 1.0);
            let g = j as f64 / (height as f64 - 1.0);
            let b = 0.25;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            img.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))
                .expect("write failed");
        }
    }
}
