use crate::utility::degrees_to_radians;
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
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin,
        }
    }
}

/// Creates a new camera struct
/// lookfrom: position of the camera
/// lookat: position to look at
/// vup: up vector
/// vfov: Vertical field of view in degrees
/// aspect_ratio: aspect ratio of the image
pub fn camera_creator(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
    let theta = degrees_to_radians(vfov);
    let h = f64::tan(theta / 2.0);
    let viewport_height = 2.0 * h;
    let viewport_width = aspect_ratio * viewport_height;

    let w = unit_vector(lookfrom - lookat);
    let u = unit_vector(cross(&vup, &w));
    let v = cross(&w, &u);

    let origin = lookfrom;
    let horizontal = viewport_width * u;
    let vertical = viewport_height * v;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

    Camera {
        origin,
        horizontal,
        vertical,
        lower_left_corner
    }
}