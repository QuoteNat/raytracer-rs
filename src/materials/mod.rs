use super::hit::*;
use super::ray::Ray;
use super::vec3::*;

pub mod lambertian;
pub use lambertian::Lambertian;
pub mod metal;
pub use metal::Metal;

pub struct ScatterStruct {
    pub attenuation: Rc<Color>,
    pub scattered: Rc<Ray>,
}
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord)-> Option<ScatterStruct>;
}