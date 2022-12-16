use crate::vec3::*;

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    /// Return position of ray at time t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}