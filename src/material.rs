use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

trait Scatterable {
    fn scatter(&self, r: Ray, rec: &HitRecord, attenuation: Vec3) -> Option<(Vec3, Ray)>;
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, r: Ray, rec: &HitRecord, attentuation: Vec3) -> Option<(Vec3, Ray)> {
        match self {
            Material::Metal(x) => x.scatter(r, rec, attentuation),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Scatterable for Metal {
    fn scatter(&self, r: Ray, rec: &HitRecord, _colour: Vec3) -> Option<(Vec3, Ray)> {
        let reflected = r.direction.unit().reflect(rec.norm);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected,
        };
        if scattered.direction.dot(&rec.norm) > 0.0 {
            Some((rec.norm, scattered))
        } else {
            None
        }
    }
}
