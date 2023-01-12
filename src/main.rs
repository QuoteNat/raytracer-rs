mod aabb;
mod buffer;
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

use std::env;
use std::io;
use std::io::Write;

use buffer::Buffer;
use camera::PerspectiveCamera;
use hit::*;
use lights::LightList;
use ray::Ray;
use scene::Scene;
use utility::*;
use vector::{write_color, Color};

use crate::{camera::Camera, lights::PointLight, vector::quick_vec};

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
