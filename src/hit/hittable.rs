use crate::vector::vec3::*;
use super::*;

/// Hit record class
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        let front_face = dot(&r.direction, outward_normal) < 0.0;
        if front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = *outward_normal * -1.0;
        }
    }
}

/// An object that can be intersected by a ray
pub trait Hittable {
    /// Implements ray intersect function for a given object
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}