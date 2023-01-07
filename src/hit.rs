use super::ray::Ray;
use super::materials::Material;
pub use std::rc::Rc;
use crate::{vector::*, materials::lambertian::Lambertian};

/// Hit record class
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;

        if self.front_face {
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

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    /// Add a Hittable object to the HittableList
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    /// Check for a ray intersection with every object in the hittable list
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord {
            p: zero_vec(),
            normal: zero_vec(),
            material: Rc::new(Lambertian {albedo: zero_vec()}),
            t: t_max,
            front_face: true,
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            match object.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    hit_anything = true;
                    closest_so_far = rec.t;
                    temp_rec.material = Rc::clone(&rec.material);
                    temp_rec.t = rec.t;
                    temp_rec.normal = rec.normal;
                    temp_rec.p = rec.p;
                    temp_rec.front_face = rec.front_face;
                }, 
                None => {},
            }
        }

        if hit_anything {
            return Some(temp_rec)
        } else {
            return None
        }
    }
}