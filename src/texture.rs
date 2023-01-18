use crate::vector::{Point3, Color};

trait Texture {
    /// Returns the color at a given texture coordinate u, v
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

struct SolidColor {
    color: Color,
}

impl SolidColor {
    /// Returns a new SolidColor with a value of color
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        self.color
    }
}