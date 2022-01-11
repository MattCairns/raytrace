use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

trait Scatterable {
    fn scatter(&self, r: Ray, rec: &HitRecord, attenuation: Vec3) -> Ray;
}

#[derive(Debug)]
pub enum Material {
    Metal(Metal),
}

impl Scatterable for Material {
    fn scatter(&self, r: Ray, rec: &HitRecord, attenuation: Vec3) -> Ray {
        match self {
            Material::Metal(x) => x.scatter(r, rec, attenuation),
        }
    }
}

#[derive(Debug, Default)]
pub struct Metal {
    attenuation: Vec3,
}

impl Scatterable for Metal {
    fn scatter(&self, r: Ray, rec: &HitRecord, attenuation: Vec3) -> Ray {
        todo!()
    }
}
