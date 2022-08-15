#![deny(clippy::all)]
#![forbid(unsafe_code)]

// This seems incorrect.
use crate::util;
use util::{Point3, Ray, Vec3};

#[derive(Copy, Clone, Debug, Default)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + (self.horizontal * u) + (self.vertical * v)
                - self.origin,
        }
    }
}
