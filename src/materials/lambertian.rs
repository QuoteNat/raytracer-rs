use super::super::hit::*;
use super::super::ray::Ray;
use super::super::vector::*;
use super::*;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord)-> Option<ScatterStruct> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Rc::new(Ray {origin: rec.p, direction: scatter_direction});
        let attenuation = Rc::new(self.albedo);
        let scatter_str = ScatterStruct {
            attenuation,
            scattered,
        };
        return Some(scatter_str);
    }
}