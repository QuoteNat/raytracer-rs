use crate::vec3::*;

/// Hit record class
struct hit_record {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl hit_record {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        let front_face = dot(r.direction(), outward_normal) < 0;
        if front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

/// An object that can be intersected by a ray
trait Hittable {
    /// Implements ray intersect function for a given object
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &mut Rec) -> bool;
}