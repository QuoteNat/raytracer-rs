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
mod materials;
use materials::*;
mod scenes;
use crate::camera::camera_creator;
use crate::materials::dielectrics::Dielectric;


/// Return ray color
fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // let mut rec = HitRecord {
    //     p: Point3{e:[0.0,0.0,0.0]}, 
    //     normal: Point3{e:[0.0,0.0,0.0]}, 
    //     t: INFINITY, 
    //     material: Rc::new(Lambertian {albedo: zero_vec()})
    // };

    // Recursion limit
    if depth <= 0 {
        return zero_vec();
    }

    match world.hit(r, 0.001, INFINITY) {
        Some(rec) => {
            match rec.material.scatter(r, &rec) {
                Some(scatter) => {
                    return *scatter.attenuation * ray_color(&scatter.scattered, world, depth-1);
                },
                None => {}
            }
        },
        None => {}
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
    let samples_per_pixel = 10;
    let max_depth = 50;

    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };
    
    let world = scenes::bubble::make_bubble();

    // Camera
    let cam = camera_creator(90.0, aspect_ratio);

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color {e:[0.0, 0.0, 0.0]};
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_float_1()) / (image_width + 1) as f64;
                let v = (j as f64 + random_float_1()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!("\nDone")
}
