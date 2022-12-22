use crate::hit::*;
use crate::materials::*;
use crate::quick_vec;
use crate::Sphere;
use crate::utility::*;

pub fn make_red_blue() -> HittableList {
    let mut world = HittableList {
        objects: Vec::new(),
    };
    let r = f64::cos(PI/4.0);

    let material_left = Rc::new(Lambertian {albedo: quick_vec(0.0, 0.0, 1.0)});
    let material_right = Rc::new(Lambertian {albedo: quick_vec(1.0, 0.0, 0.0)});

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