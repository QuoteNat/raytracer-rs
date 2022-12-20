use crate::vec3::*;

/// Hit record class
struct hit_record {
    p: Point3,
    normal: Vec3,
    t: f64,
}

trait Hittable {
    /// Implements ray intersect function for a given object
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &mut Rec) -> bool;
}