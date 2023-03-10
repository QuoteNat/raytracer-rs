use std::sync::Arc;

use super::materials::Material;
use super::ray::Ray;
use crate::aabb::AABB;
use crate::texture::TextureCoord;
use crate::{materials::Diffuse, vector::*};

/// Hit record class
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
    pub uv: TextureCoord,
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
pub trait Hittable: Sync + Send {
    /// Implements ray intersect function for a given object
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }
    /// Add a Hittable object to the HittableList
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    /// Check for a ray intersection with every object in the hittable list
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = HitRecord {
            p: zero_vec(),
            normal: zero_vec(),
            material: Arc::new(Diffuse::new_from_color(zero_vec(), 0.5)),
            t: t_max,
            front_face: true,
            uv: TextureCoord::new(0.0, 0.0),
        };
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            match object.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    hit_anything = true;
                    closest_so_far = rec.t;
                    temp_rec.material = Arc::clone(&rec.material);
                    temp_rec.t = rec.t;
                    temp_rec.normal = rec.normal;
                    temp_rec.p = rec.p;
                    temp_rec.front_face = rec.front_face;
                }
                None => {}
            }
        }

        if hit_anything {
            return Some(temp_rec);
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> AABB {
        let mut temp_box = AABB::new(zero_vec(), zero_vec());

        for object in &self.objects {
            temp_box = AABB::surround(&temp_box, &object.bounding_box());
        }

        return temp_box;
    }
}
