#![allow(dead_code)]
mod aabb;
mod background;
mod buffer;
mod bvh;
mod camera;
mod hit;
mod instance;
mod lights;
mod materials;
mod perlin;
mod ray;
mod scene;
mod scenes;
mod shapes;
mod texture;
mod utility;
mod vector;
mod volumes;

use std::env;
use std::time::Instant;

use hit::*;
use ray::Ray;
use scene::Scene;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene_file = &args[1];
    let scene = Scene::read_scene_file(scene_file);

    // Render
    //println!("P3\n{} {}\n255\n", image_width, image_height);
    let start = Instant::now();
    scene.render();
    let elapsed_time = start.elapsed().as_secs_f64();
    println!("Render time was {} s", elapsed_time);

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!("\nDone")
}
