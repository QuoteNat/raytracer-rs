use crate::vec3::*;
use crate::hittable::*;

struct Sphere {
    center: Point3,
    radius: Point3,
}

impl Hittable for Sphere {
    /// Ray intersect function for spheres
    /// Returns true if there is an intersection in range [t_min, t_max]
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: &mut Rec) -> bool {
        let oc = r.origin - *center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - radius*radius;
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            return false;
        } 
        let sqrtd = discriminant.sqrt();

        // Fin the nearest root that lies in the acceptable range
        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - center) / radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}