use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::rc::Rc;

use json;

use crate::background::{Background, BackgroundColor, GradientY};
use crate::buffer::Buffer;
use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::camera::PerspectiveCamera;
use crate::hit::{Hittable, HittableList};
use crate::lights::{LightList, PointLight};
use crate::materials::{BlinnPhong, Dielectric, Diffuse, Emissive, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::shapes::{Sphere, Triangle, XYRect};
use crate::texture::{Checker, ImageTexture, NoiseTexture, SolidColor, Texture};
use crate::utility::{random_float_1, INFINITY};
use crate::vector::{quick_vec, zero_vec, Color, Vec3};

pub struct Scene {
    camera: Box<dyn Camera>,
    pub objects: Box<HittableList>,
    pub lights: Box<LightList>,
    width: i32,
    height: i32,
    samples: i32,
    max_depth: i32,
    background: Box<dyn Background>,
    bvh_root: BVHNode,
}

impl Scene {
    pub fn new<'a>(
        camera: Box<dyn Camera>,
        objects: Box<HittableList>,
        lights: Box<LightList>,
        width: i32,
        height: i32,
        samples: i32,
        max_depth: i32,
        background: Box<dyn Background>,
    ) -> Scene {
        let bvh_root = BVHNode::new(&objects.objects, 0);
        Scene {
            camera,
            objects,
            lights,
            width,
            height,
            samples,
            max_depth,
            background,
            bvh_root,
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

        match self.bvh_root.hit(r, 0.001, INFINITY) {
            Some(rec) => {
                return rec.material.apply(r, &rec, &self, depth);
            }
            None => {}
        }

        return self.background.apply(r.direction);
    }

    #[allow(unused_variables)]
    pub fn any_hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        match self.bvh_root.hit(r, 0.001, INFINITY) {
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
        //eprintln!("{}, {}, {}", split[0], split[1], split[2]);
        quick_vec(
            split[0].parse::<f64>().unwrap(),
            split[1].parse::<f64>().unwrap(),
            split[2].parse::<f64>().unwrap(),
        )
    }

    /// Parses in the json scenefile at path
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

        // CAMERA PARSING
        let width;
        let height;
        let samples;
        let max_depth;
        let camera: Box<dyn Camera>;
        let parsed_camera = &parsed["camera"];
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

        let background_parsed = &parsed["background"];
        let background_type = match background_parsed["type"].as_str() {
            Some(string) => string,
            None => "",
        };
        let background: Box<dyn Background> = match background_type {
            "gradientY" => {
                let color1 = Scene::string_to_vec(background_parsed["color1"].as_str().unwrap());
                let color2 = Scene::string_to_vec(background_parsed["color2"].as_str().unwrap());
                Box::new(GradientY::new(color1, color2))
            }
            "backgroundColor" => {
                let color = Scene::string_to_vec(background_parsed["color"].as_str().unwrap());
                Box::new(BackgroundColor::new(color))
            }
            "" => Box::new(BackgroundColor::new(zero_vec())),
            _ => Box::new(BackgroundColor::new(zero_vec())),
        };

        // LIGHT PARSING
        let mut lights = LightList::new();
        let parsed_lights = &parsed["lights"];
        if parsed_lights.has_key("pointLight") {
            for entry in parsed_lights["pointLight"].members() {
                let position = Scene::string_to_vec(entry["position"].as_str().unwrap());
                let color = Scene::string_to_vec(entry["color"].as_str().unwrap());
                lights.add(Rc::new(PointLight { position, color }));
            }
        }

        let mut textures: HashMap<String, Rc<dyn Texture>> = HashMap::new();
        let parsed_textures = &parsed["textures"];

        if parsed.has_key("textures") {
            for entry in parsed["textures"].members() {
                match entry["type"].as_str().unwrap() {
                    "color" => {
                        let name = entry["name"].as_str().unwrap().to_string();
                        let color = Scene::string_to_vec(entry["color"].as_str().unwrap());
                        textures.insert(name, Rc::new(SolidColor::new(color)));
                    }
                    "checker" => {
                        let name = entry["name"].as_str().unwrap().to_string();
                        let odd = Rc::clone(&textures[&entry["odd"].as_str().unwrap().to_string()]);
                        let even =
                            Rc::clone(&textures[&entry["even"].as_str().unwrap().to_string()]);
                        textures.insert(name, Rc::new(Checker::new_from_textures(&odd, &even)));
                    }
                    "noise" => {
                        let name = entry["name"].as_str().unwrap().to_string();
                        let scale = entry["scale"].as_f64().unwrap();
                        textures.insert(name, Rc::new(NoiseTexture::new(scale)));
                    }
                    "image" => {
                        let name = entry["name"].as_str().unwrap().to_string();
                        let path = entry["path"].as_str().unwrap().to_string();
                        textures.insert(name, Rc::new(ImageTexture::new(path)));
                    }
                    _ => {}
                }
            }
        }
        // TODO: Possibly change textures so that they are initialized in the order they appear in the json file, and not by type.
        // This would better support textures that use other textures.
        if parsed_textures.has_key("color") {
            for entry in parsed_textures["color"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                let color = Scene::string_to_vec(entry["color"].as_str().unwrap());
                textures.insert(name, Rc::new(SolidColor::new(color)));
            }
        }

        // Note: Since checker textures can point to other textures, they should be initialized last. This is gonna be a problem later.
        if parsed_textures.has_key("checker") {
            for entry in parsed_textures["checker"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                let odd = Rc::clone(&textures[&entry["odd"].as_str().unwrap().to_string()]);
                let even = Rc::clone(&textures[&entry["even"].as_str().unwrap().to_string()]);
                textures.insert(name, Rc::new(Checker::new_from_textures(&odd, &even)));
            }
        }

        // MATERIAL PARSING
        // Materials hashmap. Keys will be used later to add materials to shapes.
        let mut materials: HashMap<String, Rc<dyn Material>> = HashMap::new();
        let parsed_materials = &parsed["materials"];

        // Parse lambertian materials
        if parsed_materials.has_key("lambertian") {
            for entry in parsed_materials["lambertian"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                let albedo = Scene::string_to_vec(entry["albedo"].as_str().unwrap());
                materials.insert(name, Rc::new(Lambertian::new(albedo)));
            }
        }

        // Parse Blinn-Phong materials
        if parsed_materials.has_key("blinnPhong") {
            for entry in parsed_materials["blinnPhong"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                let diffuse = Scene::string_to_vec(entry["diffuse"].as_str().unwrap());
                let specular = Scene::string_to_vec(entry["specular"].as_str().unwrap());
                let phong_exp = entry["phongExp"].as_f64().unwrap();
                materials.insert(name, Rc::new(BlinnPhong::new(diffuse, specular, phong_exp)));
            }
        }

        // Parse dielectric materials
        if parsed_materials.has_key("dielectric") {
            for entry in parsed_materials["dielectric"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                let ir = entry["ir"].as_f64().unwrap();
                materials.insert(name, Rc::new(Dielectric { ir }));
            }
        }

        // Parse metal materials
        if parsed_materials.has_key("metal") {
            for entry in parsed_materials["metal"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                //let albedo = Scene::string_to_vec(entry["albedo"].as_str().unwrap());
                let fuzz = entry["fuzz"].as_f64().unwrap();
                let texture = entry["texture"].as_str().unwrap();
                materials.insert(
                    name,
                    Rc::new(Metal {
                        albedo: Rc::clone(&textures[texture]),
                        fuzz,
                    }),
                );
            }
        }

        // Parse diffuse materials
        if parsed_materials.has_key("diffuse") {
            for entry in parsed_materials["diffuse"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                // let albedo = Scene::string_to_vec(entry["albedo"].as_str().unwrap());
                let absorbance = entry["absorbance"].as_f64().unwrap();
                let texture = entry["texture"].as_str().unwrap();
                materials.insert(
                    name,
                    Rc::new(Diffuse::new(Rc::clone(&textures[texture]), absorbance)),
                );
            }
        }

        // Prase emissive materials
        if parsed_materials.has_key("emissive") {
            for entry in parsed_materials["emissive"].members() {
                let name = entry["name"].as_str().unwrap().to_string();
                let texture = entry["texture"].as_str().unwrap();
                materials.insert(name, Rc::new(Emissive::new(&textures[texture])));
            }
        }

        // SHAPE PARSING
        let mut objects = HittableList {
            objects: Vec::new(),
        };
        let parsed_objects = &parsed["objects"];
        // triangles
        if parsed_objects.has_key("triangle") {
            for entry in parsed_objects["triangle"].members() {
                let point1 = Scene::string_to_vec(entry["p1"].as_str().unwrap());
                let point2 = Scene::string_to_vec(entry["p2"].as_str().unwrap());
                let point3 = Scene::string_to_vec(entry["p3"].as_str().unwrap());
                let material = entry["material"].as_str().unwrap().to_string();
                let triangle = Triangle {
                    point1,
                    point2,
                    point3,
                    material: Rc::clone(&materials[&material]),
                };

                objects.add(Rc::new(triangle));
            }
        }

        // sphere
        if parsed_objects.has_key("sphere") {
            for entry in parsed_objects["sphere"].members() {
                let center = Scene::string_to_vec(entry["center"].as_str().unwrap());
                let radius = entry["radius"].as_f64().unwrap();
                let material = entry["material"].as_str().unwrap().to_string();
                let sphere = Sphere {
                    center,
                    radius,
                    material: Rc::clone(&materials[&material]),
                };

                objects.add(Rc::new(sphere));
            }
        }

        // XYRect
        if parsed_objects.has_key("xyrect") {
            for entry in parsed_objects["xyrect"].members() {
                let x0 = entry["x0"].as_f64().unwrap();
                let x1 = entry["x1"].as_f64().unwrap();
                let y0 = entry["y0"].as_f64().unwrap();
                let y1 = entry["y1"].as_f64().unwrap();
                let k = entry["z"].as_f64().unwrap();
                let material = entry["material"].as_str().unwrap().to_string();
                let material = &Rc::clone(&materials[&material]);
                let rect = XYRect::new(x0, x1, y0, y1, k, material);

                objects.add(Rc::new(rect));
            }
        }

        eprintln!("{} lights", lights.len());
        eprintln!("{} textures", textures.len());
        eprintln!("{} materials", materials.len());
        eprintln!("{} objects", objects.len());
        let bvh_root = BVHNode::new(&objects.objects, 0);
        // eprintln!("{}")
        Scene {
            camera,
            lights: Box::new(lights),
            objects: Box::new(objects),
            width,
            height,
            samples,
            max_depth,
            background,
            bvh_root,
        }
    }

    pub fn render(&self) {
        let mut buffer = Buffer::new(self.width as u32, self.height as u32);
        for j in (0..self.height).rev() {
            eprint!("\rScanlines remaining: {} ", j);
            io::stderr().flush().unwrap();

            for i in 0..self.width {
                let mut pixel_color = Color { e: [0.0, 0.0, 0.0] };
                for _ in 0..self.samples {
                    let u = (i as f64 + random_float_1()) / (self.width + 1) as f64;
                    let v = (j as f64 + random_float_1()) / (self.height - 1) as f64;
                    let r = self.get_ray(u, v);
                    pixel_color += self.ray_color(&r, self.max_depth);
                }

                buffer.write(
                    (1.0 / self.samples as f64) * pixel_color,
                    i as u32,
                    j as u32,
                );
            }
        }
        println!();
        buffer.buffer_to_png(String::from("image.png"));
    }
}
