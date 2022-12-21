use super::super::hit::*;
use super::super::ray::Ray;
use super::super::vector::*;
use super::Material;
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
        *scattered = Ray{origin: rec.p, direction: reflected};
        *attenuation = self.albedo;
        return dot(&scattered.direction, &rec.normal) > 0.0;
    }
}