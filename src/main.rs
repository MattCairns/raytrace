use image::imageops::rotate180;
use image::{ImageBuffer, RgbImage};
use raytrace::hittable::HittableList;
use raytrace::material::Lambertian;
use raytrace::material::Material;
use raytrace::material::Metal;
use raytrace::material::Scatterable;
use raytrace::ray::Ray;
use raytrace::scene::{Camera, Screen};
use raytrace::sphere::Sphere;
use raytrace::util::rand_f64;
use raytrace::util::write_color;
use raytrace::vec3::Vec3;

fn ray_color(r: &Ray, world: &HittableList, depth: u16) -> Vec3 {
    if depth <= 0 {
        Vec3::ZEROES
    } else {
        match world.hit(r, 0.001, f64::INFINITY) {
            Some(hit) => {
                // let scattered = Ray::default();
                // let target = hit.p + hit.norm + rand_unit_vec3();
                let scatter = hit.mat.scatter(*r, &hit);
                match scatter {
                    Some(dat) => dat.0 * ray_color(&dat.1, world, depth - 1),
                    None => Vec3::ZEROES,
                }
            }
            None => {
                let unit_dir = r.direction.unit();
                let t = 0.5 * (unit_dir.y + 1.0);
                Vec3::ONES * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}

fn main() {
    let screen = Screen::new(16.0 / 9.0, 200);
    let samples = 100;
    let max_depth = 10;
    let mut world = HittableList { hittables: vec![] };
    let left = Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Material::Metal(Metal {
            albedo: Vec3::new(0.8, 0.8, 0.8),
            blur: 0.3,
        }),
    };

    let centre = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat: Material::Metal(Metal {
            albedo: Vec3::new(0.7, 0.3, 0.3),
            blur: 0.0,
        }),
    };

    let right = Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Material::Metal(Metal {
            albedo: Vec3::new(0.8, 0.8, 0.2),
            blur: 1.0,
        }),
    };

    let ground = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: Material::Lambertian(Lambertian {
            albedo: Vec3::new(0.8, 0.8, 0.0),
        }),
    };

    world.hittables.push(left);
    world.hittables.push(centre);
    world.hittables.push(right);
    world.hittables.push(ground);

    let cam = {
        let viewport_height = 2.0;
        Camera::new(viewport_height, screen.aspect_ratio * viewport_height, 1.0)
    };

    // let mut img = std::fs::File::create("test.ppm").expect("Failed to create image");
    let mut img: RgbImage = ImageBuffer::new(screen.width, screen.height);

    for j in (0..screen.height).rev() {
        println!(
            "\rRendering...  {}%",
            ((1.0 - (j as f64 / screen.height as f64)) * 100.0) as u8
        );
        for i in 0..screen.width {
            let mut pixel_color = Vec3::ZEROES;
            for _ in 0..samples {
                let u = (i as f64 + rand_f64()) / (screen.width as f64 - 1.0);
                let v = (j as f64 + rand_f64()) / (screen.height as f64 - 1.0);

                let r = cam.ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }

            write_color(&mut img, i, j, pixel_color, samples);
        }
    }
    rotate180(&img).save("test.png").unwrap();
}
