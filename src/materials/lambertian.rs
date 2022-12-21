use super::super::hit::*;
use super::super::ray::Ray;
use super::super::vector::*;
use super::Material;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray {origin: rec.p, direction: scatter_direction};
        self.albedo.err_write();
        *attenuation = self.albedo;
        attenuation.err_write();
        return true;
    }
}