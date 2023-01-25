use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::AABB,
    hit::{HitRecord, Hittable},
    ray::Ray,
};

/// BVH Node struct, for creating a bounding volume hierarchy
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb: AABB,
}

impl BVHNode {
    /// Compares two boxes along an axis (0=x, 1=y, 2=z)
    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: u32) -> Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();

        if box_a.min().e[axis as usize] + box_a.max().e[axis as usize] / 2.0
            == box_b.min().e[axis as usize] + box_b.max().e[axis as usize] / 2.0
        {
            return Ordering::Equal;
        }
        // we sort by the center between the max and min on an axis because this is arbitrary and it still achieves the same effect
        match box_a.min().e[axis as usize] + box_a.max().e[axis as usize] / 2.0
            < box_b.min().e[axis as usize] + box_b.max().e[axis as usize] / 2.0
        {
            true => return Ordering::Less,
            false => return Ordering::Greater,
        }
    }

    pub fn new(src_objects: &Vec<Arc<dyn Hittable>>, axis: u32) -> BVHNode {
        let mut objects = src_objects.clone();
        objects.sort_by(|a, b| BVHNode::box_compare(a, b, axis));

        let length = objects.len();

        if length == 1 {
            BVHNode {
                left: Arc::clone(&objects[0]),
                right: Arc::clone(&objects[0]),
                aabb: src_objects[0].bounding_box(),
            }
        } else if length == 2 {
            BVHNode {
                left: Arc::clone(&objects[0]),
                right: Arc::clone(&objects[1]),
                aabb: AABB::surround(&objects[0].bounding_box(), &objects[1].bounding_box()),
            }
        } else {
            let mid = length / 2;

            let left: Arc<dyn Hittable> =
                Arc::new(BVHNode::new(&objects[0..mid].to_vec(), (axis + 1) % 3));
            let right: Arc<dyn Hittable> =
                Arc::new(BVHNode::new(&objects[mid..length].to_vec(), (axis + 1) % 3));

            BVHNode {
                left: Arc::clone(&left),
                right: Arc::clone(&right),
                aabb: AABB::surround(&left.bounding_box(), &right.bounding_box()),
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);

        // TODO: Find a cleaner solution for this
        if hit_left.is_some() {
            let hit_left = hit_left.unwrap();
            let hit_right = self.right.hit(r, t_min, hit_left.t);
            if hit_right.is_some() {
                let hit_right = hit_right.unwrap();
                if hit_right.t < hit_left.t {
                    return Some(hit_right);
                } else {
                    return Some(hit_left);
                }
            } else {
                return Some(hit_left);
            }
        } else {
            let hit_right = self.right.hit(r, t_min, t_max);
            // At this point hit_left is none, so returning hit_right should contain the correct result regardless
            return hit_right;
        }
    }

    fn bounding_box(&self) -> AABB {
        return self.aabb;
    }
}
