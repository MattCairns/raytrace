use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    pub hittables: Vec<Sphere>,
}

impl HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord::default();

        for ob in &self.hittables {
            let (hit, temp_rec) = ob.hit(r, t_min, closest_so_far);
            if hit {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec = temp_rec;
            }
        }
        (hit_anything, rec)
    }
}

#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: &Vec3) {
        self.front_face = r.direction.dot(outward_norm) < 0.0;
        if self.front_face {
            self.norm = *outward_norm
        } else {
            self.norm = -*outward_norm
        };
    }
}

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, HitRecord) {
        let oc = r.origin - self.center;
        let a = r.direction.dot(&r.direction);
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        let roota = (-b - discriminant.sqrt()) / a;
        let rootb = (-b + discriminant.sqrt()) / a;

        let mut rec = HitRecord::default();
        if discriminant < 0.0 {
            (false, rec)
        } else if rootb < t_min || rootb >= t_max {
            (false, rec)
        } else {
            if roota < t_min || roota >= t_max {
                rec.t = roota;
            } else {
                rec.t = rootb
            }
            rec.p = r.at(rec.t);
            let outward_norm = (rec.p - self.center) / self.radius;
            rec.set_face_normal(r, &outward_norm);
            rec.norm = (rec.p - self.center) / self.radius;
            (true, rec)
        }
    }
}
