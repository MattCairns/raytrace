use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Default)]
pub struct HittableList {
    pub hittables: Vec<Sphere>,
}

impl HittableList {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec = HitRecord::default();

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

#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.len_sqr();
        let b = oc.dot(&r.direction);
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant >= 0.0 {
            let sqrtd = discriminant.sqrt();
            let roota = (-b - sqrtd) / a;
            let rootb = (-b + sqrtd) / a;

            for root in [roota, rootb].iter() {
                if *root < t_max && *root > t_min {
                    let t = *root;
                    let p = r.at(*root);
                    let norm = (p - self.center) / self.radius;
                    let front_face = r.direction.dot(&norm) < 0.0;
                    return Some(HitRecord {
                        t,
                        p,
                        norm: if front_face { norm } else { -norm },
                        front_face,
                    });
                }
            }
        }
        None
    }
}
