use super::vec3::*;
use super::super::utility::*;

pub fn random_vec_1() -> Vec3 {
    Vec3 {e:[random_float_1(),
        random_float_1(),
        random_float_1()
        ]}
}

pub fn random_vec(min: f64, max: f64) -> Vec3 {
    Vec3 {e:[random_float(min, max),
        random_float(min, max),
        random_float(min, max)
        ]}
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec(-1.0, 1.0);
        if p.length_squared() < 1.0 {return p;}
    }
}