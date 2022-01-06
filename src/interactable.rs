use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, mut rec: HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        let roota = (-b - discriminant.sqrt()) / a;
        let rootb = (-b + discriminant.sqrt()) / a;

        if discriminant < 0.0 {
            false
        } else if rootb < t_min || rootb >= t_max {
            false
        } else {
            if roota < t_min || roota >= t_max {
                rec.t = roota;
            } else {
                rec.t = rootb
            }
            rec.p = r.at(rec.t);
            rec.norm = (rec.p - self.center) / self.radius;
            true
        }
    }
}
