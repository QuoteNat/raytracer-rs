use std::sync::Arc;

use crate::lights::Light;
use crate::scene::Scene;
use crate::texture::{SolidColor, Texture};

use super::hit::*;
use super::ray::Ray;
use super::vector::*;

pub trait Material: Sync + Send {
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene, depth: i32) -> Color;
}

pub struct Metal {
    pub albedo: Arc<dyn Texture>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzz: f64) -> Metal {
        Metal {
            albedo: Arc::clone(&albedo),
            fuzz,
        }
    }

    pub fn new_from_color(color: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: Arc::new(SolidColor::new(color)),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene, depth: i32) -> Color {
        let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
        let scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * random_in_unit_sphere(),
        };

        return self.albedo.value(&rec.uv, &rec.p) * scene.ray_color(&scattered, depth);
    }
}

#[derive(Clone)]
pub struct Diffuse {
    albedo: Arc<dyn Texture>,
    absorbance: f64,
}

impl Diffuse {
    pub fn new(albedo: Arc<dyn Texture>, absorbance: f64) -> Diffuse {
        Diffuse {
            albedo: Arc::clone(&albedo),
            absorbance,
        }
    }

    pub fn new_from_color(color: Color, absorbance: f64) -> Diffuse {
        Diffuse {
            albedo: Arc::new(SolidColor::new(color)),
            absorbance,
        }
    }
}

impl Material for Diffuse {
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene, depth: i32) -> Color {
        // lambertian light contribution
        let cr =
            self.albedo.value(&rec.uv, &rec.p) * scene.lights.apply(r_in, rec, scene).contribution;

        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray {
            origin: rec.p,
            direction: scatter_direction,
        };

        let scattered_color = scene.ray_color(&scattered, depth);

        return cr * self.absorbance
            + (1.0 - self.absorbance) * self.albedo.value(&rec.uv, &rec.p) * scattered_color;
    }
}

/// Dielectric material
pub struct Dielectric {
    /// Index of refraction
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene, depth: i32) -> Color {
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

        // Full reflection
        if cannot_refract {
            let direction = reflect(&unit_direction, &rec.normal);
            let ray = Ray {
                origin: rec.p,
                direction,
            };
            return scene.ray_color(&ray, depth);
        } else {
            // get reflectance ratio based
            let reflectance = reflectance(cos_theta, refraction_ratio);
            // reflect color
            let reflect_direction = reflect(&unit_direction, &rec.normal);
            let reflect_ray = Ray {
                origin: rec.p,
                direction: reflect_direction,
            };
            let reflect_color = scene.ray_color(&reflect_ray, depth);
            // refract color
            let refract_direction = refract(&unit_direction, &rec.normal, refraction_ratio);
            let refract_ray = Ray {
                origin: rec.p,
                direction: refract_direction,
            };
            let refract_color = scene.ray_color(&refract_ray, depth);

            // return combination of reflection and refraction
            return reflect_color * reflectance + refract_color * (1.0 - reflectance);
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene, _depth: i32) -> Color {
        // lambertian light contribution
        let cr = self.albedo * scene.lights.apply(r_in, rec, scene).contribution;
        vec_clamp(cr, 0.0, 1.0)
    }
}

pub struct BlinnPhong {
    diffuse: Color,
    specular: Color,
    phong_exp: f64,
}

impl BlinnPhong {
    pub fn new(diffuse: Color, specular: Color, phong_exp: f64) -> BlinnPhong {
        BlinnPhong {
            diffuse,
            specular,
            phong_exp,
        }
    }
}

impl Material for BlinnPhong {
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene, _depth: i32) -> Color {
        let mut l_r = zero_vec();
        for light in &scene.lights.lights {
            let detail = light.as_ref().apply(r_in, rec, scene);
            let l = unit_vector(detail.position - rec.p);
            let v = unit_vector(-1.0 * r_in.direction);
            let half = unit_vector(l + v);
            let n_dot_h_to_p = f64::powf(dot(&rec.normal, &half), self.phong_exp);
            let spec_component = self.specular * f64::max(0.0, n_dot_h_to_p);

            l_r = l_r + (self.diffuse + spec_component) * detail.contribution;
        }

        return vec_clamp(l_r, 0.0, 1.0);
    }
}

pub struct Emissive {
    emit: Arc<dyn Texture>,
}

impl Emissive {
    /// Creates a new Emissive material from a texture
    pub fn new(texture: &Arc<dyn Texture>) -> Emissive {
        Emissive {
            emit: Arc::clone(texture),
        }
    }
}

impl Material for Emissive {
    fn apply(&self, _r_in: &Ray, rec: &HitRecord, _scene: &Scene, _depth: i32) -> Color {
        self.emit.value(&rec.uv, &rec.p)
    }
}
