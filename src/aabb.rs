use std::mem::swap;

use crate::{
    ray::Ray,
    vector::{zero_vec, Point3},
};

/// Axis aligned bounding box
#[derive(Copy, Clone)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    /// Creates a new AABB from minimum to maximum
    pub fn new(minimum: Point3, maximum: Point3) -> AABB {
        AABB { minimum, maximum }
    }

    /// Creates a new AABB that bounds a set of points
    pub fn new_from_points(points: &Vec<Point3>) -> AABB {
        let mut maximum = zero_vec();
        let mut minimum = zero_vec();
        for point in points {
            for i in 0..3 {
                if point[i] < minimum[i] {
                    minimum[i] = point[i];
                } else if point[i] > maximum[i] {
                    maximum[i] = point[i];
                }
            }
        }

        AABB::new(minimum, maximum)
    }

    /// Returns the minimum point of the AABB
    pub fn min(&self) -> Point3 {
        self.minimum
    }
    /// Returns the maximum point of the AABB
    pub fn max(&self) -> Point3 {
        self.maximum
    }

    /// Check if a ray intersects the AABB
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.minimum[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1)
            }
            let t_min = match t0 > t_min {
                true => t0,
                false => t_min,
            };
            let t_max = match t1 < t_max {
                true => t1,
                false => t_max,
            };
            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }

    /// Create a bounding box that encompasses two bounding boxes
    pub fn surround(box1: &AABB, box2: &AABB) -> AABB {
        // Reusing functions, can always use the more optimized code from the book later (this is a lie I won't rememer to do this)
        AABB::new_from_points(&vec![
            box1.minimum,
            box1.maximum,
            box2.minimum,
            box2.maximum,
        ])
    }
}
