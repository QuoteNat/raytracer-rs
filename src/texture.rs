use crate::vector::{Point3, Color};

pub trait Texture {
    /// Returns the color at a given texture coordinate u, v
    fn value(&self, uv: &TextureCoord, p: &Point3) -> Color;
}

pub struct TextureCoord {
    pub u: f64,
    pub v: f64,
}

impl TextureCoord {
    pub fn new(u: f64, v: f64) -> TextureCoord {
        TextureCoord { u, v }
    }
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    /// Returns a new SolidColor with a value of color
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }
}

impl Texture for SolidColor {
    #[allow(unused_variables)]
    fn value(&self, uv: &TextureCoord, p: &Point3) -> Color {
        self.color
    }
}