use std::sync::Arc;

use crate::{
    aabb::AABB,
    hit::{HitRecord, Hittable},
    ray::Ray,
    vector::Vec3,
};

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(ptr: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate {
            ptr: Arc::clone(&ptr),
            offset,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> AABB {
        let aabb = self.ptr.bounding_box();

        AABB::new(aabb.min() + self.offset, aabb.max() + self.offset)
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved = Ray::new(r.origin + self.offset, r.direction);

        match self.ptr.hit(&moved, t_min, t_max) {
            Some(rec) => {
                let mut new_rec = rec.clone();
                new_rec.p += self.offset;
                new_rec.set_face_normal(&moved, &rec.normal);

                return Some(new_rec);
            }
            None => return None,
        }
    }
}
