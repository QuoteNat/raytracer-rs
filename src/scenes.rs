use std::sync::Arc;

use crate::background::BackgroundColor;

use crate::bvh::BVHNode;
use crate::camera::PerspectiveCamera;

use crate::hit::*;
use crate::instance::RotateY;
use crate::instance::Translate;
use crate::lights::LightList;
use crate::materials::*;

use crate::scene::Scene;
use crate::shapes::Box;
use crate::shapes::Sphere;

use crate::shapes::XZRect;
use crate::texture::ImageTexture;
use crate::texture::NoiseTexture;
use crate::texture::SolidColor;
use crate::texture::Texture;
use crate::utility::*;
use crate::vector::Color;
use crate::vector::Point3;
use crate::vector::Vec3;

use crate::volumes::ConstantMedium;

// pub fn make_red_blue() -> HittableList {
//     let absorbance = 0.5;
//     let mut world = HittableList {
//         objects: Vec::new(),
//     };
//     let r = f64::cos(PI / 4.0);

//     let material_left = Rc::new(Diffuse {
//         albedo: quick_vec(1.0, 1.0, 1.0),
//         absorbance,
//     });
//     let material_right = Rc::new(Diffuse {
//         albedo: quick_vec(1.0, 1.0, 1.0),
//         absorbance,
//     });

//     world.add(Rc::new(Sphere {
//         center: quick_vec(-r, 0.0, -1.0),
//         radius: r,
//         material: material_left,
//     }));
//     world.add(Rc::new(Sphere {
//         center: quick_vec(r, 0.0, -1.0),
//         radius: r,
//         material: material_right,
//     }));

//     world
// }

// pub fn random_scene() -> HittableList {
//     // World
//     let mut world = HittableList {
//         objects: Vec::new(),
//     };

//     let absorbance = 0.5;
//     let ground_material: Arc<dyn Material> = Arc::new(Diffuse {
//         albedo: quick_vec(0.5, 0.5, 0.5),
//         absorbance,
//     });
//     world.add(Rc::new(Sphere {
//         center: quick_vec(0.0, -1000.0, 0.0),
//         radius: 1000.0,
//         material: ground_material,
//     }));

//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat = random_float_1();
//             let center = quick_vec(
//                 a as f64 + 0.9 * random_float_1(),
//                 0.2,
//                 b as f64 + 0.9 * random_float_1(),
//             );

//             if (center - quick_vec(4.0, 0.2, 0.0)).length() > 0.9 {
//                 let sphere_material: Rc<dyn Material>;

//                 if choose_mat < 0.8 {
//                     // diffuse
//                     let albedo = random_vec_1();
//                     sphere_material = Rc::new(Diffuse { albedo, absorbance });
//                     world.add(Rc::new(Sphere {
//                         center,
//                         radius: 0.2,
//                         material: sphere_material,
//                     }))
//                 } else if choose_mat < 0.95 {
//                     // metal
//                     let albedo = random_vec(0.5, 1.0);
//                     let fuzz = random_float(0.0, 0.5);
//                     sphere_material = Rc::new(Metal { albedo, fuzz });

//                     world.add(Rc::new(Sphere {
//                         center,
//                         radius: 0.2,
//                         material: sphere_material,
//                     }))
//                 } else {
//                     // glass
//                     sphere_material = Rc::new(Dielectric { ir: 1.5 });

//                     world.add(Rc::new(Sphere {
//                         center,
//                         radius: 0.2,
//                         material: sphere_material,
//                     }))
//                 }
//             }
//         }
//     }

//     let material1 = Rc::new(Dielectric { ir: 1.5 });
//     world.add(Rc::new(Sphere {
//         center: quick_vec(0.0, 1.0, 0.0),
//         radius: 1.0,
//         material: material1,
//     }));

//     let material2 = Rc::new(Diffuse {
//         albedo: quick_vec(0.4, 0.2, 0.1),
//         absorbance,
//     });
//     world.add(Rc::new(Sphere {
//         center: quick_vec(-4.0, 1.0, 0.0),
//         radius: 1.0,
//         material: material2,
//     }));

//     let material3 = Rc::new(Metal {
//         albedo: quick_vec(0.7, 0.6, 0.5),
//         fuzz: 0.0,
//     });
//     world.add(Rc::new(Sphere {
//         center: quick_vec(4.0, 1.0, 0.0),
//         radius: 1.0,
//         material: material3,
//     }));

//     world
// }

// pub fn random_scene_camera(aspect_ratio: f64) -> RTOWCamera {
//     let lookfrom = quick_vec(13.0, 2.0, 3.0);
//     let lookat = quick_vec(0.0, 0.0, 0.0);
//     let vup = quick_vec(0.0, 1.0, 0.0);
//     let dist_to_focus = 10.0;
//     let aperture = 0.1;

//     // Return camera
//     RTOWCamera::new(
//         lookfrom,
//         lookat,
//         vup,
//         20.0,
//         aspect_ratio,
//         aperture,
//         dist_to_focus,
//     )
// }

impl Scene {
    pub fn gen_final_scene() -> Scene {
        let mut objects = HittableList::new();
        let absorbance = 0.0;
        let ground: Arc<dyn Material> = Arc::new(Diffuse::new_from_color(
            Color::new(0.48, 0.83, 0.53),
            absorbance,
        ));

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let i = i as f64;
                let j = j as f64;
                let w = 100.0;
                let x0 = -1000.0 + i * w;
                let z0 = -1000.0 + j * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = random_float(1.0, 101.0);
                let z1 = z0 + w;

                objects.add(Arc::new(Box::new(
                    &Point3::new(x0, y0, z0),
                    &Point3::new(x1, y1, z1),
                    Arc::clone(&ground),
                )));
            }
        }

        let light_texture: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(7.0, 7.0, 7.0)));
        let light: Arc<dyn Material> = Arc::new(Emissive::new(&light_texture));
        objects.add(Arc::new(XZRect::new(
            123.0, 423.0, 147.0, 412.0, 554.0, &light,
        )));

        let center = Point3::new(400.0, 400.0, 400.0);
        let not_moving_sphere_material: Arc<dyn Material> = Arc::new(Diffuse::new_from_color(
            Color::new(0.7, 0.3, 0.1),
            absorbance,
        ));
        objects.add(Arc::new(Sphere {
            center,
            radius: 50.0,
            material: Arc::clone(&not_moving_sphere_material),
        }));

        let dielectric: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

        objects.add(Arc::new(Sphere {
            center: Point3::new(260.0, 150.0, 45.0),
            radius: 50.0,
            material: Arc::clone(&dielectric),
        }));
        objects.add(Arc::new(Sphere {
            center: Point3::new(0.0, 150.0, 145.0),
            radius: 50.0,
            material: Arc::new(Metal::new_from_color(Color::new(0.8, 0.8, 0.9), 1.0)),
        }));

        let boundary: Arc<dyn Hittable> = Arc::new(Sphere::new(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            &dielectric,
        ));
        objects.add(Arc::clone(&boundary));
        let fog_color: Arc<dyn Texture> = Arc::new(SolidColor::new(Color::new(0.2, 0.4, 0.9)));
        objects.add(Arc::new(ConstantMedium::new(
            &Arc::clone(&boundary),
            0.2,
            &fog_color,
            absorbance,
        )));
        let pertext = Arc::new(NoiseTexture::new(0.1));
        let permat = Arc::new(Diffuse::new(pertext, absorbance));
        objects.add(Arc::new(Sphere {
            center: Point3::new(220.0, 280.0, 300.0),
            radius: 80.0,
            material: permat,
        }));
        let emat = Arc::new(Diffuse::new(
            Arc::new(ImageTexture::new("assets/Blue_Marble_2002.png".to_string())),
            absorbance,
        ));
        objects.add(Arc::new(Sphere {
            center: Point3::new(400.0, 200.0, 400.0),
            radius: 100.0,
            material: emat,
        }));

        let mut boxes2 = HittableList::new();
        let white = Arc::new(SolidColor::new(Color::new(0.73, 0.73, 0.73)));
        let white: Arc<dyn Material> = Arc::new(Diffuse::new(white, absorbance));
        let ns = 1000;
        for _ in 0..ns {
            let rand1 = random_float(0.0, 165.0);
            let rand2 = random_float(0.0, 165.0);
            let rand3 = random_float(0.0, 165.0);
            boxes2.add(Arc::new(Sphere::new(
                Vec3::new(rand1, rand2, rand3),
                10.0,
                &white,
            )))
        }

        let boxes2 = BVHNode::new(&boxes2.objects, 0);

        objects.add(Arc::new(Translate::new(
            Arc::new(RotateY::new(Arc::new(boxes2), 15.0)),
            Vec3::new(-100.0, 270.0, 395.0),
        )));

        let camera = Arc::new(PerspectiveCamera::new(
            Vec3::new(478.0, 278.0, -600.0),
            Vec3::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            40.0,
            1.0,
        ));

        // this was tedious
        Scene::new(
            camera,
            Arc::new(objects),
            Arc::new(LightList::new()),
            800,
            800,
            10000,
            10,
            Arc::new(BackgroundColor::new(Color::new(0.0, 0.0, 0.0))),
        )
    }
}
