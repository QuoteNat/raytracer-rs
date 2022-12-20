use vector::*;
use create::ray::Ray;

struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    fn Camera(&mut self) {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        self.origin = Point3{e:[0.0, 0.0, 0.0]};
        self.horizontal = Point3{e:[viewport_width, 0.0, 0.0]};
        self.vertical = Point3{e:[0.0, viewport_height, 0.0]};
        self.lower_left_corner = origin - horizontal/2 - vertical/2 - Vec3{e:[0.0, 0.0, focal_length]};
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin,
        }
    }
}