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

/// Return a random point in the unit sphere
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec(-1.0, 1.0);
        if p.length_squared() < 1.0 {return p;}
    }
}

/// Random unit vector
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

/// Reflect a vector around an normal
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * dot(v,n) * 2.0
}

pub fn zero_vec() -> Vec3 {
    Vec3 {e: [0.0, 0.0, 0.0]}
}

pub fn quick_vec(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 {e: [x, y, z]}
}

/// Ray refraction using Snell's law
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let uv = *uv;
    let n = *n;
    let cos_theta = f64::min(dot(&(-uv), &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;

    r_out_perp + r_out_parallel
}