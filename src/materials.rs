use crate::utility::random_float_1;

use super::hit::*;
use super::ray::Ray;
use super::vector::*;

pub struct ScatterStruct {
    pub attenuation: Rc<Color>,
    pub scattered: Rc<Ray>,
}
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterStruct>;
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterStruct> {
        let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
        let scattered = Rc::new(Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_in_unit_sphere(),
        });
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

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterStruct> {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Rc::new(Ray {
            origin: rec.p,
            direction: scatter_direction,
        });
        let attenuation = Rc::new(self.albedo);
        let scatter_str = ScatterStruct {
            attenuation,
            scattered,
        };
        return Some(scatter_str);
    }
}

/// Dielectric material
pub struct Dielectric {
    /// Index of refraction
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterStruct> {
        // Set attenuation to full for all
        let attenuation = Rc::new(quick_vec(1.0, 1.0, 1.0));

        // Let refraction ratio equal 1/ir if outside the object, or ir if inside the object
        let refraction_ratio;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        };

        // Get the unit vector of the ray direction
        let unit_direction = unit_vector(r_in.direction);
        // Get cos of the angle between -unit_direction and the normal vector (set to 1.0 if over 1.0 somehow)
        let cos_theta = f64::min(dot(&(-unit_direction), &rec.normal), 1.0);
        // Get the sin of the angle
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        // If refraction_ratio * sin_theta is greater than 1, refraction is not possible
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        // If refraction is not possible, or if reflectance is greater than a random f64 from 0 to 1
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float_1() {
            // Reflect
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            // Refract (bug is probably here)
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        // Return the scattered ray
        let scattered = Rc::new(Ray {
            origin: rec.p,
            direction,
        });

        return Some(ScatterStruct {
            attenuation,
            scattered,
        });
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}
