use super::hit::*;
use super::ray::Ray;
use super::vec3::*;

pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}