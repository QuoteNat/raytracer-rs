use super::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    /// Ray intersect function for spheres
    /// Returns true if there is an intersection in range [t_min, t_max]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - (self.radius*self.radius);
        let discriminant = half_b*half_b - a*c;
        
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Fin the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }


        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let material = Rc::clone(&self.material);

        let mut rec = HitRecord {
            t,
            p,
            normal,
            material
        };

        rec.set_face_normal(r, &normal);

        return Some(rec);
    }
}