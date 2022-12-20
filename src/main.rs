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
mod camera;
use crate::camera::CameraCreator;


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
    let samples_per_pixel = 100;

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
    let cam = CameraCreator();

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height-1).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color {e:[0.0, 0.0, 0.0]};
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_float_1()) / (image_width + 1) as f64;
                let v = (j as f64 + random_float_1()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!("\nDone")
}
