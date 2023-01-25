use std::sync::Arc;

use crate::scene::Scene;
use crate::vector::{dot, unit_vector, vec_clamp, zero_vec, Color, Point3, Vec3};
use crate::HitRecord;
use crate::Ray;

pub struct LightDetails {
    pub contribution: Color,
    pub position: Point3,
}

/// Abstract Light trait.
pub trait Light: Sync + Send {
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene) -> LightDetails;
}

/// List of lights in a scene
pub struct LightList {
    pub lights: Vec<Arc<dyn Light>>,
}

impl LightList {
    /// Creates a new empty LightList
    pub fn new() -> LightList {
        LightList { lights: Vec::new() }
    }

    /// Adds a light to the light list
    pub fn add(&mut self, object: Arc<dyn Light>) {
        self.lights.push(object);
    }

    pub fn len(&self) -> usize {
        self.lights.len()
    }
}

impl Light for LightList {
    /// Calculates light contribution from all lights in the scene
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene) -> LightDetails {
        let mut cont = zero_vec();

        for light in &self.lights {
            cont = cont + light.apply(r_in, rec, scene).contribution;
        }

        LightDetails {
            contribution: vec_clamp(cont, 0.0, 1.0),
            position: zero_vec(),
        }
    }
}

/// Point light
pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
}

impl Light for PointLight {
    /// Returns light contribution based off of lambert's law in the form of a Color
    #[allow(unused_variables)]
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene) -> LightDetails {
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
            return LightDetails {
                contribution: zero_vec(),
                position: self.position,
            };
        }

        return LightDetails {
            contribution: self.color * f64::max(0.0, dot(&n, &l)),
            position: self.position,
        };
    }
}
