use crate::hit::{Hittable, HittableList};
use crate::lights::{Light, LightList};
use crate::ray::Ray;
use crate::utility::INFINITY;
use crate::vector::{unit_vector, vec_clamp, zero_vec, Color};
use crate::Camera;

pub struct Scene<'a> {
    camera: Box<dyn Camera>,
    pub objects: &'a HittableList,
    pub lights: &'a LightList,
}

impl Scene<'_> {
    pub fn new<'a>(
        camera: Box<dyn Camera>,
        objects: &'a HittableList,
        lights: &'a LightList,
    ) -> Scene<'a> {
        Scene {
            camera,
            objects,
            lights,
        }
    }

    /// Return ray color
    ///
    /// r: Ray
    ///
    /// depth: Current recursion depth as an int. Decrementing will be handled by the function so no need to do it in Material classes.
    pub fn ray_color(&self, r: &Ray, depth: i32) -> Color {
        let depth = depth - 1;
        // Recursion limit
        if depth <= 0 {
            return zero_vec();
        }

        match self.objects.hit(r, 0.001, INFINITY) {
            Some(rec) => {
                return rec.material.apply(r, &rec, &self, depth);
            }
            None => {}
        }

        let unit_direction = unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Color { e: [1.0, 1.0, 1.0] } + t * Color { e: [0.5, 0.7, 1.0] };
        //return zero_vec();
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        self.camera.get_ray(s, t)
    }
}
