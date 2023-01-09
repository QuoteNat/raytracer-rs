use crate::scene::Scene;
use crate::vector::{dot, unit_vector, vec_clamp, zero_vec, Color, Vec3};
use crate::Ray;
use crate::{HitRecord, HittableList};
use std::rc::Rc;

/// Abstract Light trait.
pub trait Light {
    fn contribution(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene) -> Color;
}

/// List of lights in a scene
pub struct LightList {
    lights: Vec<Rc<dyn Light>>,
}

impl LightList {
    /// Creates a new empty LightList
    pub fn new() -> LightList {
        LightList { lights: Vec::new() }
    }

    /// Adds a light to the light list
    pub fn add(&mut self, object: Rc<dyn Light>) {
        self.lights.push(object);
    }
}

impl Light for LightList {
    /// Calculates light contribution from all lights in the scene
    fn contribution(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene) -> Color {
        let mut cont = zero_vec();

        for light in &self.lights {
            cont = cont + light.contribution(r_in, rec, scene);
        }

        vec_clamp(cont, 0.0, 1.0)
    }
}

/// Point light
pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
}

impl Light for PointLight {
    /// Returns light contribution based off of lambert's law in the form of a Color
    fn contribution(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene) -> Color {
        let n = rec.normal;
        let l = unit_vector(self.position - rec.p);

        // Return zero if an object obstructs
        if scene.any_hit(
            &Ray {
                origin: rec.p,
                direction: l,
            },
            0.001,
            (self.position - rec.p).length(),
        ) {
            return zero_vec();
        }

        return self.color * f64::max(0.0, dot(&n, &l));
    }
}
