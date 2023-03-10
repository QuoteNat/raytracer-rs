use crate::vector::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
    /// Return position of ray at time t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
