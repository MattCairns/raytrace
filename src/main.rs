use std::io::Write;
fn main() {
    let width = 256;
    let height = 256;

    let mut img = std::fs::File::create("test.ppm").expect("Failed to create image");

    img.write_fmt(format_args!("P3\n{} {}\n255\n", width, height))
        .expect("write failed");
}
