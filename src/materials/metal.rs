use super::super::hit::*;
use super::super::ray::Ray;
use super::super::vector::*;
use super::*;
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord)-> Option<ScatterStruct> {
        let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
        let scattered = Rc::new(Ray{origin: rec.p, direction: reflected + self.fuzz*random_in_unit_sphere()});
        let attenuation = Rc::new(self.albedo);
        let scatter_str = ScatterStruct {
            attenuation: Rc::clone(&attenuation),
            scattered: Rc::clone(&scattered),
        };

        if dot(&scattered.direction, &rec.normal) > 0.0 {
            return Some(scatter_str);
        } else {
            return None;
        }
    }
}