use std::io;
use std::io::Write;
mod vector;
use vector::*;
mod ray;
use ray::Ray;
mod shapes;
use shapes::*;
mod hit;
use hit::*;
mod utility;
use utility::*;


/// Return ray color
fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord { p: Point3{e:[0.0,0.0,0.0]}, normal: Point3{e:[0.0,0.0,0.0]}, t: INFINITY };

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (Color{e:[1.0, 1.0, 1.0]} + rec.normal);
    }

    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color{e:[1.0, 1.0, 1.0]} + t * Color{e:[0.5, 0.7, 1.0]};
}

fn main() {
    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };
    world.add(&Sphere {
        center: Point3{e:[0.0, 0.0, -1.0]},
        radius: 0.5,
    });
    world.add(&Sphere {
        center: Point3{e:[0.0, -100.5, -1.0]},
        radius: 100.0,
    });

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

    for j in (-1..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;
            let r = Ray {
                origin: origin,
                direction: lower_left_corner + u*horizontal + v*vertical - origin
            };
            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!("\nDone")
}
