use raytrace::interactable::{HitRecord, HittableList, Sphere};
use raytrace::ray::Ray;
use raytrace::vec3::{write_color, Vec3};
use std::io::Write;

fn ray_color(r: &Ray, world: &HittableList) -> Vec3 {
    // let hit = hit_sphere(&Vec3::new(0.0, 0.0, -2.0), 0.5, r);
    //
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
    let ratio = 16.0 / 9.0;
    let width = 1000 as u32;
    let height = (width as f32 / ratio) as u32;

    let mut world = HittableList::default();
    let s1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    println!("{:?}", s1);
    let s2 = Sphere {
        center: Vec3::new(0.0, -100.0, -5.0),
        radius: 100.0,
    };
    world.hittables.push(s1);
    world.hittables.push(s2);

    let viewport_height = 2.0;
    let viewport_width = ratio * viewport_height;
    let focal_len = 1.0;

    let orig = Vec3::ZEROES;
    let horiz = Vec3::new(viewport_width as f64, 0.0, 0.0);
    let vert = Vec3::new(0.0, viewport_height as f64, 0.0);

    let lower_left_corner = orig - horiz / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_len);

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

            let pixel_color = ray_color(&r, &world);

            write_color(&img, pixel_color);
        }
    }
}
