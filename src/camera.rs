use crate::vector::*;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        }
    }
}

/// Creates a new camera struct
pub fn camera_creator() -> Camera {
    let aspect_ratio = 16.0 / 9.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3{e:[0.0, 0.0, 0.0]};
    let horizontal = Point3{e:[viewport_width, 0.0, 0.0]};
    let vertical = Point3{e:[0.0, viewport_height, 0.0]};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{e:[0.0, 0.0, focal_length]};

    Camera {
        origin,
        horizontal,
        vertical,
        lower_left_corner
    }
}