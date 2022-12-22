use crate::vector::vec_utility::{quick_vec, refract, reflect};
use crate::random_float_1;
use super::*;

/// Dielectric material
pub struct Dielectric {
    /// Index of refraction
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord)-> Option<ScatterStruct> {
        let attenuation = Rc::new(quick_vec(1.0, 1.0, 1.0));
        let mut refraction_ratio = 0.0;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        };

        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = f64::min(dot(&(-unit_direction), &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float_1() {
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        let scattered = Rc::new(Ray {
            origin: rec.p,
            direction,
        });

        return Some(ScatterStruct { attenuation, scattered });
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0-r0) * f64::powi(1.0 - cosine, 5)
}