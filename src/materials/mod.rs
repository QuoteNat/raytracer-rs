use super::hit::*;
use super::ray::Ray;
use super::vec3::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color, scattered: &Ray) -> bool;
}