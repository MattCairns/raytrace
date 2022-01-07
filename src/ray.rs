use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at() {
        let r = Ray {
            origin: Vec3::new(1.0, 1.0, 1.0),
            direction: Vec3::new(2.0, 2.0, 2.0),
        };
        assert_eq!(r.at(2.0), Vec3::new(5.0, 5.0, 5.0));
    }
}
