use crate::{vector::*, materials::lambertian::Lambertian};

use super::*;

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