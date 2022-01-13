use crate::util::{rand_unit_vec3, rand_vec3_in_sphere};
use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Scatterable {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Metal(Metal),
    Lambertian(Lambertian),
}

impl Scatterable for Material {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Metal(x) => x.scatter(r, rec),
            Material::Lambertian(x) => x.scatter(r, rec),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Metal {
    pub albedo: Vec3,
    pub blur: f64,
}

impl Scatterable for Metal {
    fn scatter(&self, r: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = r.direction.unit().reflect(rec.norm);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected * rand_vec3_in_sphere() * self.blur,
        };
        if scattered.direction.dot(&rec.norm) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _r: Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let scattered = Ray {
            origin: rec.p,
            direction: rec.norm + rand_unit_vec3(),
        };
        if scattered.direction.dot(&rec.norm) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
