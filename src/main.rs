use std::io;
use std::io::Write;
mod shapes;
mod vector;
use vector::{unit_vector, vec_clamp, write_color, zero_vec, Color};
mod ray;
use ray::Ray;
mod hit;
use hit::*;
mod utility;
use utility::*;
mod camera;
use camera::PerspectiveCamera;
mod lights;
mod materials;
mod scene;
mod scenes;
use lights::{Light, LightList};
use scene::Scene;

use crate::{camera::Camera, lights::PointLight, vector::quick_vec};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 10;
    let max_depth = 10;

    let world = scenes::make_bubble();
    let mut lights = LightList::new();
    lights.add(Rc::new(PointLight {
        position: quick_vec(0.0, 2.0, 0.25),
        color: quick_vec(1.0, 1.0, 1.0),
    }));
    // lights.add(Rc::new(PointLight {
    //     position: quick_vec(1.0, -1.0, 0.25),
    //     color: quick_vec(0.0, 1.0, 0.0),
    // }));
    // lights.add(Rc::new(PointLight {
    //     position: quick_vec(-1.0, -1.0, 0.25),
    //     color: quick_vec(0.0, 0.0, 1.0),
    // }));

    let cam = PerspectiveCamera::new(
        quick_vec(0.0, 0.0, 0.5),
        quick_vec(0.0, 0.0, -1.0),
        quick_vec(0.0, 1.0, 0.0),
        90.0,
        1.9,
    );

    let scene = Scene::new(Box::new(cam), &world, &lights);

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Color { e: [0.0, 0.0, 0.0] };
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_float_1()) / (image_width + 1) as f64;
                let v = (j as f64 + random_float_1()) / (image_height - 1) as f64;
                let r = scene.get_ray(u, v);
                pixel_color += scene.ray_color(&r, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!("\nDone")
}
