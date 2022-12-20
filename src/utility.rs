extern crate rand;
use rand::thread_rng;
use rand::Rng;

/// Max value of f64
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

/// Converts degrees to radians
pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

/// Generates a random float in the range [min, max]
pub fn random_float(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(min..max)
}

pub fn random_float_1() -> f64 {
    let mut rng = thread_rng();
    return rng.gen_range(0.0..1.0)
}