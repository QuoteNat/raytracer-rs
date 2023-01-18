#![allow(dead_code)]
mod aabb;
mod buffer;
mod bvh;
mod camera;
mod hit;
mod lights;
mod materials;
mod ray;
mod scene;
mod scenes;
mod shapes;
mod utility;
mod vector;
mod texture;

use std::env;

use hit::*;
use ray::Ray;
use scene::Scene;

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene_file = &args[1];
    let scene = Scene::read_scene_file(scene_file);

    // Render
    //println!("P3\n{} {}\n255\n", image_width, image_height);

    scene.render();

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!("\nDone")
}
