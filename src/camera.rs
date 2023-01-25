use crate::ray::Ray;
use crate::utility::{degrees_to_radians, random_in_unit_disk};
use crate::vector::*;

/// Trait for a Camera class that generates rays for the scene
pub trait Camera: Sync + Send {
    fn get_ray(&self, s: f64, t: f64) -> Ray;
}

/// Simple perspective camera
pub struct PerspectiveCamera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl PerspectiveCamera {
    /// Creates a new PerspectiveCamera
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> PerspectiveCamera {
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

        PerspectiveCamera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let origin = self.origin;
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;

        Ray { origin, direction }
    }
}

/// Camera struct as described in the Ray Tracing in One Weekend Book
#[derive(Copy, Clone)]
pub struct RTOWCamera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl RTOWCamera {
    /// Creates a new camera struct
    /// lookfrom: position of the camera
    /// lookat: position to look at
    /// vup: up vector
    /// vfov: Vertical field of view in degrees
    /// aspect_ratio: aspect ratio of the image
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> RTOWCamera {
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        RTOWCamera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }
}

impl Camera for RTOWCamera {
    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        return Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        };
    }
}
