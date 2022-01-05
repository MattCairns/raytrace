use std::cmp;
use std::fmt;
use std::ops;

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub trait Vec3Properties {
    fn len(&self) -> f64;
    fn len_sqr(&self) -> f64;
    fn dot(&self, v: &Vec3) -> f64;
    fn cross(&self, v: &Vec3) -> Vec3;
    fn unit(&self) -> Vec3;
}

impl Vec3Properties for Vec3 {
    fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn dot(&self, v: &Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.x
    }

    fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    fn unit(&self) -> Vec3 {
        self / self.len()
    }
}

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
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

impl cmp::PartialEq for Vec3 {
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_sqr_is_correct() {
        let v = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(v.len_sqr(), 3.0);
    }

    #[test]
    fn len_is_correct() {
        let v = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let val = 3.0 as f64;
        assert_eq!(v.len(), val.sqrt());
    }

    #[test]
    fn dot() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let b = Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };

        assert_eq!(a.dot(&b), 6.0);
    }

    #[test]
    fn cross() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let b = Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let result = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(a.cross(&b), result);
    }

    #[test]
    fn unit() {
        let v = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };
        let result = Vec3 {
            x: v.x / v.len(),
            y: v.y / v.len(),
            z: v.z / v.len(),
        };
        assert_eq!(v.unit(), result);
    }

    #[test]
    fn add() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };

        let b = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };

        assert_eq!(
            a + b,
            Vec3 {
                x: 5.0,
                y: 5.0,
                z: 5.0
            }
        );
    }

    #[test]
    fn div() {
        let a = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };

        assert_eq!(
            a / 2.0,
            Vec3 {
                x: 2.0,
                y: 2.0,
                z: 2.0
            }
        );
    }
}
