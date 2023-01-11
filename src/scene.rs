use std::fs::File;
use std::io::Read;
use std::path::Path;

use json::{object, JsonValue};

use crate::camera::PerspectiveCamera;
use crate::hit::{Hittable, HittableList};
use crate::lights::{Light, LightList};
use crate::ray::Ray;
use crate::utility::INFINITY;
use crate::vector::{quick_vec, unit_vector, vec_clamp, zero_vec, Color, Vec3};
use crate::Camera;

pub struct Scene<'a> {
    camera: Box<dyn Camera>,
    pub objects: &'a HittableList,
    pub lights: &'a LightList,
    width: i32,
    height: i32,
    samples: i32,
    max_depth: i32,
}

impl Scene<'_> {
    pub fn new<'a>(
        camera: Box<dyn Camera>,
        objects: &'a HittableList,
        lights: &'a LightList,
        width: i32,
        height: i32,
        samples: i32,
        max_depth: i32,
    ) -> Scene<'a> {
        Scene {
            camera,
            objects,
            lights,
            width,
            height,
            samples,
            max_depth,
        }
    }

    /// Return ray color
    ///
    /// r: Ray
    ///
    /// depth: Current recursion depth as an int. Decrementing will be handled by the function so no need to do it in Material classes.
    pub fn ray_color(&self, r: &Ray, depth: i32) -> Color {
        let depth = depth - 1;
        // Recursion limit
        if depth <= 0 {
            return zero_vec();
        }

        match self.objects.hit(r, 0.001, INFINITY) {
            Some(rec) => {
                return rec.material.apply(r, &rec, &self, depth);
            }
            None => {}
        }

        let unit_direction = unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y() + 1.0);
        //return (1.0 - t) * Color { e: [1.0, 1.0, 1.0] } + t * Color { e: [0.5, 0.7, 1.0] };
        return zero_vec();
    }

    pub fn any_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        match self.objects.hit(r, 0.001, INFINITY) {
            Some(_) => return true,
            None => return false,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        self.camera.get_ray(s, t)
    }

    /// Converts a string to a vector.
    ///
    /// Syntax is "[x] [y] [z]" to vector of (x, y, z)
    fn string_to_vec(string: &str) -> Vec3 {
        let split = string.split(" ");
        let split: Vec<&str> = split.collect();
        quick_vec(
            split[0].parse::<f64>().unwrap(),
            split[1].parse::<f64>().unwrap(),
            split[2].parse::<f64>().unwrap(),
        )
    }

    pub fn read_scene_file(path: &String) -> Scene {
        // Read in scene file
        let path = Path::new(path);
        let display = path.display();
        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => {}
        };

        let parsed = match json::parse(&s) {
            Err(why) => panic!("couldn't convert {} to json: {}", display, why),
            Ok(parsed) => parsed,
        };

        // Create camera
        let mut width = 0;
        let mut height = 0;
        let mut samples = 0;
        let mut max_depth = 0;
        let camera: Box<dyn Camera>;
        let parsed_camera = parsed["camera"];
        match parsed_camera["type"].as_str().unwrap() {
            "perspective" => {
                // TODO: Add actual match statements to this (to be procrastinated until the heat death of the universe)
                let lookfrom = Scene::string_to_vec(parsed_camera["lookfrom"].as_str().unwrap());
                let lookat = Scene::string_to_vec(parsed_camera["lookat"].as_str().unwrap());
                let vup = Scene::string_to_vec(parsed_camera["vup"].as_str().unwrap());
                let vfov = parsed_camera["vfov"].as_f64().unwrap();
                width = parsed_camera["width"].as_i32().unwrap();
                height = parsed_camera["height"].as_i32().unwrap();
                samples = parsed_camera["samples"].as_i32().unwrap();
                max_depth = parsed_camera["max_depth"].as_i32().unwrap();
                let aspect_ratio = width as f64 / height as f64;

                camera = Box::new(PerspectiveCamera::new(
                    lookfrom,
                    lookat,
                    vup,
                    vfov,
                    aspect_ratio,
                ));
            }
            _ => {
                panic!("Camera not defined")
            }
        };

        let parsed_lights = parsed["lights"];

        Scene {}
    }
}
