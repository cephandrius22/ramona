use crate::util::{Color, HitRecord, Ray, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Color, Ray)>;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };

        return Some((self.albedo, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction.unit_vector().reflect(rec.normal);
        let scattered = Ray {origin: rec.p, direction: reflected};
        let attenuation = self.albedo;

        if scattered.direction * rec.normal > 0.0 {
            return Some((attenuation, scattered));
        } else {
            return None;
        }
    }
}