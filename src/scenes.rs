use crate::camera::RTOWCamera;
use crate::hit::*;
use crate::materials::*;
use crate::shapes::Sphere;
use crate::utility::*;
use crate::vector::{quick_vec, random_vec, random_vec_1};

pub fn make_red_blue() -> HittableList {
    let mut world = HittableList {
        objects: Vec::new(),
    };
    let r = f64::cos(PI / 4.0);

    let material_left = Rc::new(Lambertian {
        albedo: quick_vec(0.0, 0.0, 1.0),
    });
    let material_right = Rc::new(Lambertian {
        albedo: quick_vec(1.0, 0.0, 0.0),
    });

    world.add(Rc::new(Sphere {
        center: quick_vec(-r, 0.0, -1.0),
        radius: r,
        material: material_left,
    }));
    world.add(Rc::new(Sphere {
        center: quick_vec(r, 0.0, -1.0),
        radius: r,
        material: material_right,
    }));

    world
}

pub fn random_scene() -> HittableList {
    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: quick_vec(0.5, 0.5, 0.5),
    });
    world.add(Rc::new(Sphere {
        center: quick_vec(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float_1();
            let center = quick_vec(
                a as f64 + 0.9 * random_float_1(),
                0.2,
                b as f64 + 0.9 * random_float_1(),
            );

            if (center - quick_vec(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = random_vec_1();
                    sphere_material = Rc::new(Lambertian { albedo });
                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random_vec(0.5, 1.0);
                    let fuzz = random_float(0.0, 0.5);
                    sphere_material = Rc::new(Metal { albedo, fuzz });

                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }))
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric { ir: 1.5 });

                    world.add(Rc::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }))
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric { ir: 1.5 });
    world.add(Rc::new(Sphere {
        center: quick_vec(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    }));

    let material2 = Rc::new(Lambertian {
        albedo: quick_vec(0.4, 0.2, 0.1),
    });
    world.add(Rc::new(Sphere {
        center: quick_vec(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    }));

    let material3 = Rc::new(Metal {
        albedo: quick_vec(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere {
        center: quick_vec(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    }));

    world
}

pub fn random_scene_camera(aspect_ratio: f64) -> RTOWCamera {
    let lookfrom = quick_vec(13.0, 2.0, 3.0);
    let lookat = quick_vec(0.0, 0.0, 0.0);
    let vup = quick_vec(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    // Return camera
    RTOWCamera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    )
}

pub fn make_bubble() -> HittableList {
    // World
    let mut world = HittableList {
        objects: Vec::new(),
    };

    let material_ground: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: quick_vec(0.8, 0.8, 0.0),
    });
    let material_center: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: quick_vec(0.1, 0.2, 0.5),
    });
    let material_left: Rc<dyn Material> = Rc::new(Dielectric { ir: 1.5 });
    let material_right: Rc<dyn Material> = Rc::new(Metal {
        albedo: quick_vec(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    let sphere_ground: Rc<dyn Hittable> = Rc::new(Sphere {
        center: quick_vec(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::clone(&material_ground),
    });
    let sphere_center: Rc<dyn Hittable> = Rc::new(Sphere {
        center: quick_vec(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::clone(&material_center),
    });
    let sphere_left: Rc<dyn Hittable> = Rc::new(Sphere {
        center: quick_vec(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::clone(&material_left),
    });
    let sphere_bubble: Rc<dyn Hittable> = Rc::new(Sphere {
        center: quick_vec(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: Rc::clone(&material_left),
    });
    let sphere_right: Rc<dyn Hittable> = Rc::new(Sphere {
        center: quick_vec(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::clone(&material_right),
    });

    world.add(Rc::clone(&sphere_ground));
    world.add(Rc::clone(&sphere_center));
    world.add(Rc::clone(&sphere_left));
    world.add(Rc::clone(&sphere_bubble));
    world.add(Rc::clone(&sphere_right));

    world
}
