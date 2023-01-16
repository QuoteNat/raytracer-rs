use crate::aabb::AABB;
use crate::hit::*;
use crate::materials::Material;
use crate::vector::*;
use crate::Ray;

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
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

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
            material,
            front_face: true,
        };

        rec.set_face_normal(r, &normal);

        return Some(rec);
    }

    fn bounding_box(&self) -> AABB {
        let center = self.center;
        let rad_vec = Vec3::new(self.radius, self.radius, self.radius);

        AABB::new(center - rad_vec, center + rad_vec)
    }
}

pub struct Triangle {
    pub point1: Point3,
    pub point2: Point3,
    pub point3: Point3,
    pub material: Rc<dyn Material>,
}

impl Hittable for Triangle {
    /// Ray triangle intersection
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // based on https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution.html
        // compute the plane's normal
        let v1v2 = self.point2 - self.point1;
        let v1v3 = self.point3 - self.point1;
        // no need to normalize
        let n = cross(&v1v2, &v1v3);
        //let area2 = N.length();

        // Find P

        // check if ray and plane are parallel.
        let n_dot_ray_direction = dot(&n, &r.direction);
        if f64::abs(n_dot_ray_direction) < t_min {
            return None;
        }

        // compute d parameter using equation 2
        let d = dot(&-n, &self.point1);

        // compute t
        let t = -(dot(&n, &r.origin) + d) / n_dot_ray_direction;

        // check if triangle is behind the ray
        if t < t_min {
            return None;
        }

        // compute the intersection point
        let p = r.origin + t * r.direction;

        // Inside outside test
        // edge 1
        let edge1 = self.point2 - self.point1;
        let vp1 = p - self.point1;
        let c = cross(&edge1, &vp1);
        if dot(&n, &c) < 0.0 {
            return None;
        }

        // edge 2
        let edge2 = self.point3 - self.point2;
        let vp2 = p - self.point2;
        let c = cross(&edge2, &vp2);
        if dot(&n, &c) < 0.0 {
            return None;
        }

        // edge 3 (due to how barycentric coordinates work, checking the third coordinate should be unnessecary. Check this later.)
        let edge3 = self.point1 - self.point3;
        let vp3 = p - self.point3;
        let c = cross(&edge3, &vp3);
        if dot(&n, &c) < 0.0 {
            return None;
        }

        // return hitstruct
        let mut rec = HitRecord {
            p,
            normal: n,
            material: Rc::clone(&self.material),
            t,
            front_face: true,
        };

        rec.set_face_normal(r, &n);

        return Some(rec);
    }

    fn bounding_box(&self) -> AABB {
        AABB::new_from_points(&vec![self.point1, self.point2, self.point3])
    }
}
