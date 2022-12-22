use crate::hit::*;
use crate::materials::*;
use crate::Dielectric;
use crate::quick_vec;
use crate::Sphere;

pub fn make_bubble() -> HittableList {
        // World
        let mut world = HittableList {
            objects: Vec::new(),
        };
        
        let material_ground: Rc<dyn Material> = Rc::new(Lambertian{albedo: quick_vec(0.8, 0.8, 0.0)});
        let material_center: Rc<dyn Material>  = Rc::new(Lambertian{albedo: quick_vec(0.1, 0.2, 0.5)});
        let material_left: Rc<dyn Material>  = Rc::new(Dielectric{ir: 1.5});
        let material_right: Rc<dyn Material>  = Rc::new(Metal{albedo: quick_vec(0.8, 0.6, 0.2), fuzz: 0.0});
    
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
            material: Rc::clone(&material_left)
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