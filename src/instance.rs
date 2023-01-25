use std::{f64::INFINITY, fs::DirEntry, sync::Arc};

use crate::{
    aabb::AABB,
    hit::{HitRecord, Hittable},
    ray::Ray,
    utility::degrees_to_radians,
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

struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    aabb: AABB,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);
        let aabb = p.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let x = i * aabb.max().x() + (1.0 - i) * aabb.min().x();
                    let y = j * aabb.max().y() + (1.0 - i) * aabb.min().y();
                    let z = k * aabb.max().z() + (1.0 - i) * aabb.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::min(min[c], tester[c]);
                    }
                }
            }
        }

        let aabb = AABB::new(min, max);

        RotateY {
            ptr: p,
            sin_theta,
            cos_theta,
            aabb,
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> AABB {
        self.aabb
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin[0] = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin[2] = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];

        direction[0] = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction[2] = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];

        let rotated_r = Ray::new(origin, direction);

        match self.ptr.hit(&rotated_r, t_min, t_max) {
            Some(rec) => {
                let mut p = rec.p;
                let mut normal = rec.normal;

                p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
                p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

                normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
                normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

                let mut rec = HitRecord {
                    p,
                    normal,
                    material: rec.material,
                    t: rec.t,
                    front_face: rec.front_face,
                    uv: rec.uv,
                };

                rec.set_face_normal(&rotated_r, &normal);

                Some(rec)
            }
            None => None,
        }
    }
}
