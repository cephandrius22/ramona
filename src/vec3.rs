#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};

use rand::Rng;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        // Not sure about this. Should maybe just be some util function outside of the impl.
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        // in the same hemisphere as normal
        if in_unit_sphere * normal > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let mut rng = rand::thread_rng();
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    pub fn near_zero(self) -> bool {
        // return true if vector is close to zero is all dims
        const S: f32 = 1e-8;
        
        f32::abs(self.x) < S &&
            f32::abs(self.y) < S &&
            f32::abs(self.z) < S
    }

    pub fn reflect(self, n: Vec3) -> Vec3 {
        self - n * ((self * n) * 2.0)
    }

    pub fn refract(self, normal: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(-self * normal, 1.0);
        let r_out_perp = (self + (normal * cos_theta)) * etai_over_etat;
        let r_out_parallel = normal * -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared()));

        r_out_perp + r_out_parallel
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Vec3) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
            self.y == other.y &&
            self.z == other.z
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vec3::new(2.0, 4.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a + b, Vec3::new(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(2.0, 4.0, 6.0);
        a += Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a, Vec3::new(3.0, 6.0, 9.0))
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(2.0, 4.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a - b, Vec3::new(1.0, 2.0, 3.0))
    }

    #[test]
    fn test_mult() {
        let a = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(a * 2.0, Vec3::new(4.0, 8.0, 12.0))
    }

    #[test]
    fn test_div() {
        let a = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(a / 2.0, Vec3::new(1.0, 2.0, 3.0))
    }

    #[test]
    fn test_neg() {
        let a = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(-a, Vec3::new(-2.0, -4.0, -6.0))
    }
}