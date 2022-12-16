use std::io;
use std::io::Write;
mod vec3;
use vec3::*;
mod color;
use color::*;
mod ray;
use ray::*;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}

/// Return ray color
fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3{e:[0.0, 0.0, -1.0]}, 0.5, r) {
        return Color{e:[1.0, 0.0, 0.0]}
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t) * Color{e:[1.0, 1.0, 1.0]} + t * Color{e:[0.5, 0.7, 1.0]}
}

fn main() {
    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3{e:[0.0, 0.0, 0.0]};
    let horizontal = Vec3{e:[viewport_width, 0.0, 0.0]};
    let vertical = Vec3{e:[0.0, viewport_height, 0.0]};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{e:[0.0, 0.0, focal_length]};

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height-1).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray {
                origin: origin,
                direction: lower_left_corner + u*horizontal + v*vertical - origin
            };
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!()
}
