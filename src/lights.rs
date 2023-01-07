use crate::vector::{Color, unit_vector, dot};

/// Abstract Light trait.
pub trait Light {
    pub fn contribution(&self, r_in: &Ray, rec: &HitRecord, hitList: &HittableList) -> Color;
}

/// Point light
pub struct PointLight {
    pub position: Vec3,
    pub color: Color
}

impl Light for PointLight {
    /// Returns light contribution based off of lambert's law in the form of a Color
    fn contribution(&self, r_in: &Ray, rec: &HitRecord, hitList: &HittableList) -> Color {
        let n = rec.normal;
        let l = unit_vector(self.position - rec.p);
        
        return self.color * f64::max(0.0, dot(n, l));
    }
}