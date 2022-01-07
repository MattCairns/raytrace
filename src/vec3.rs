use rand::prelude::*;
use std::ops;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub const ZEROES: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONES: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    pub fn rand() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    pub fn unit(&self) -> Vec3 {
        self / self.len()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_sqr_is_correct() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v.len_sqr(), 3.0);
    }

    #[test]
    fn len_is_correct() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        let val = 3.0 as f64;
        assert_eq!(v.len(), val.sqrt());
    }

    #[test]
    fn dot() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(a.dot(&b), 6.0);

        let dir = Vec3::new(1.7777777777777777, -1.0, -1.0);
        assert_eq!(dir.dot(&dir), 5.160493827160494);
    }

    #[test]
    fn cross() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(10.0, 10.0, 10.0);
        let result = Vec3::new(-10.0, 20.0, -10.0);
        assert_eq!(a.cross(&b), result);
    }

    #[test]
    fn unit() {
        let v = Vec3::new(4.0, 4.0, 4.0);
        let result = Vec3::new(v.x / v.len(), v.y / v.len(), v.z / v.len());
        assert_eq!(v.unit(), result);
    }

    #[test]
    fn add() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(a + b, Vec3::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn mul_scalar() {
        let a = Vec3::new(1.0, 1.0, 2.0);
        assert_eq!(a * 0.5, Vec3::new(0.5, 0.5, 1.0));
    }

    #[test]
    fn mul_vec3() {
        let a = Vec3::new(1.0, 1.0, 2.0);
        let b = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(a * b, Vec3::new(4.0, 4.0, 8.0));
    }

    #[test]
    fn div_scalar() {
        let a = Vec3::new(1.0, 1.0, 2.0);
        assert_eq!(a / 2.0, Vec3::new(0.5, 0.5, 1.0));
    }

    #[test]
    fn div_scalar_ref() {
        let a = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(&a / 2.0, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn div() {
        let a = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(a / 2.0, Vec3::new(2.0, 2.0, 2.0));
    }
}
