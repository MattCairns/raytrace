use raytrace::interactable::{HittableList, Sphere};
use raytrace::ray::Ray;
use raytrace::scene::{Camera, Screen};
use raytrace::util::write_color;
use raytrace::vec3::Vec3;
use std::io::Write;

fn ray_color(r: &Ray, world: &HittableList, depth: u16) -> Vec3 {
    let (hit, rec) = world.hit(r, 0.001, 9999999999999999.0);
    if depth <= 0 {
        Vec3::ZEROES
    } else if hit {
        let target = rec.p + rec.norm + Vec3::rand_range(-1.0, 1.0);
        ray_color(
            &Ray {
                origin: rec.p,
                direction: target - rec.p,
            },
            world,
            depth - 1,
        ) * 0.5
    } else {
        let unit_dir = r.direction.unit();
        let t = 0.5 * (unit_dir.y + 1.0);
        Vec3::ONES * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let screen = Screen::new(16.0 / 9.0, 400);
    let samples = 100;
    let max_depth = 50;
    let mut world = HittableList::default();
    let s1 = Sphere {
        center: Vec3::new(0.0, 0.0, -2.0),
        radius: 0.5,
    };

    let s2 = Sphere {
        center: Vec3::new(0.0, -100.0, -1.0),
        radius: 100.0,
    };
    world.hittables.push(s1);
    world.hittables.push(s2);

    let cam = {
        let viewport_height = 2.0;
        Camera::new(viewport_height, screen.aspect_ratio * viewport_height, 1.0)
    };

    let mut img = std::fs::File::create("test.ppm").expect("Failed to create image");

    img.write_fmt(format_args!(
        "P3\n{} {}\n255\n",
        screen.width, screen.height
    ))
    .expect("write failed");

    for j in (0..screen.height).rev() {
        println!(
            "\rRendering...  {}",
            (1.0 - (j as f64 / screen.height as f64)) * 100.0
        );
        for i in 0..screen.width {
            let mut pixel_color = Vec3::ZEROES;
            for _ in 0..samples {
                let u = i as f64 / (screen.width as f64 - 1.0);
                let v = j as f64 / (screen.height as f64 - 1.0);

                let r = cam.ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }

            write_color(&img, pixel_color, samples);
        }
    }
}
