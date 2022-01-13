use crate::material::{Material, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct HittableList {
    pub hittables: Vec<Sphere>,
}

impl HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord {
            p: Vec3::ZEROES,
            norm: Vec3::ZEROES,
            t: 0.0,
            front_face: true,
            mat: Material::Metal(Metal {
                albedo: Vec3::ZEROES,
                blur: 0.0,
            }),
        };

        for ob in &self.hittables {
            match ob.hit(r, t_min, closest_so_far) {
                Some(hit) => {
                    hit_anything = true;
                    closest_so_far = hit.t;
                    rec = hit;
                }
                None => {}
            }
        }
        match hit_anything {
            true => Some(rec),
            false => None,
        }
    }
}

#[derive(Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Material,
}
