use rand::Rng;

use crate::util::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };

        return Some((self.albedo, scattered));
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction.unit_vector().reflect(rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + (Vec3::random_in_unit_sphere() * self.fuzz),
        };
        let attenuation = self.albedo;

        if scattered.direction * rec.normal > 0.0 {
            return Some((attenuation, scattered));
        } else {
            return None;
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Dialetric {
    pub index_of_refraction: f32,
}

pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + ((1.0 - r0) * f32::powi(1.0 - cosine, 5))
}

impl Material for Dialetric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refration_ratio = if rec.front_face {
            (1.0 / self.index_of_refraction)
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction.unit_vector();

        let cos_theta = f32::min(-unit_direction * rec.normal, 1.0);
        let sin_theta = f32::sqrt(1.0 - (cos_theta * cos_theta));

        let mut rng = rand::thread_rng();
        let cannot_refarct = refration_ratio * sin_theta > 1.0;
        let direction = if cannot_refarct
            || reflectance(cos_theta, refration_ratio) > rng.gen_range(0.0..1.0)
        {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refration_ratio)
        };

        return Some((
            attenuation,
            Ray {
                origin: rec.p,
                direction: direction,
            },
        ));
    }
}
