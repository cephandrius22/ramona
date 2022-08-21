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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * focus_dist * viewport_width;
        let vertical = v * focus_dist * viewport_height;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (w * focus_dist);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: (aperture / 2.0),
        }
    }

    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + (self.horizontal * s) + (self.vertical * t)
                - self.origin
                - offset,
        }
    }
}
