use raytrace::interactable::{HitRecord, HittableList, Sphere};
use raytrace::ray::Ray;
use raytrace::scene::{Screen, ViewPort};
use raytrace::vec3::{write_color, Vec3};
use std::io::Write;

fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    let (hit, rec) = world.hit(r, 0.0, 9999999999999999.0);
    if hit {
        (rec.norm + Vec3::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_dir = r.direction.unit();
        let t = 0.5 * (unit_dir.y + 1.0);
        Vec3::ONES * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let screen = Screen::new(16.0 / 9.0, 400);

    let mut world = HittableList::default();
    let s1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let s2 = Sphere {
        center: Vec3::new(0.0, -100.0, -1.0),
        radius: 100.0,
    };
    world.hittables.push(s1);
    world.hittables.push(s2);

    let viewport = {
        let viewport_height = 2.0;
        ViewPort::new(viewport_height, screen.aspect_ratio * viewport_height, 1.0)
    };

    let origin = Vec3::ZEROES;
    let horiz = Vec3::new(viewport.width, 0.0, 0.0);
    let vert = Vec3::new(0.0, viewport.height, 0.0);
    let lower_left_corner =
        origin - horiz / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, viewport.focal_length);

    let mut img = std::fs::File::create("test.ppm").expect("Failed to create image");

    img.write_fmt(format_args!(
        "P3\n{} {}\n255\n",
        screen.width, screen.height
    ))
    .expect("write failed");

    for j in (0..screen.height).rev() {
        println!("\rRendering: {}/{}", j, screen.height);
        for i in 0..screen.width {
            let u = i as f64 / (screen.width as f64 - 1.0);
            let v = j as f64 / (screen.height as f64 - 1.0);
            let r = Ray {
                origin,
                direction: lower_left_corner + (horiz * u) + (vert * v) - origin,
            };

            let pixel_color = ray_color(&r, &world);

            write_color(&img, pixel_color);
        }
    }
}
