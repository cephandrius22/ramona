#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::ops::{Add, Div, Mul, Neg, Sub, AddAssign};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
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

pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f32) -> Point3 {
        self.origin + (self.direction * t)
    }
}

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // dot product of ray and outward normal tells us if we are hitting
        // the inside or ouside of the surface.
        self.front_face = (ray.direction * outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc * ray.direction;
        let c = (oc.length_squared()) - (self.radius * self.radius);

        let discriminant: f32 = (half_b * half_b) - (a * c);

        // no roots (negative discriminant) = no intersction
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        hit_record.t = root;
        hit_record.p = ray.at(hit_record.t);
        hit_record.normal = (hit_record.p - self.center) / self.radius;

        let outward_normal = (hit_record.p - self.center) / self.radius;
        hit_record.set_face_normal(&ray, outward_normal);

        true
    }
}

pub struct HittableList {
    // A vector of objects that implement the Hittable trait.
    // Box is needed because Hittable objects can be of different
    // sizes.
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        // I'm not 100% clear on if this is the correct way to do
        // this.
        self.objects.push(Box::new(object) as Box<dyn Hittable>);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec;
            }
        }

        hit_anything
    }
}
