use std::sync::Arc;

use crate::{
    buffer::Buffer,
    perlin::Perlin,
    vector::{Color, Point3},
};

pub trait Texture: Sync + Send {
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

pub struct Checker {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Checker {
    /// Creates a new Checker texture from two texture pointers
    pub fn new_from_textures(odd: &Arc<dyn Texture>, even: &Arc<dyn Texture>) -> Checker {
        Checker {
            odd: Arc::clone(odd),
            even: Arc::clone(even),
        }
    }

    /// Creates a new Checker texture from two colors
    pub fn new_from_colors(odd: Color, even: Color) -> Checker {
        Checker {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}

impl Texture for Checker {
    fn value(&self, uv: &TextureCoord, p: &Point3) -> Color {
        let sines = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());

        if sines < 0.0 {
            return self.odd.value(uv, p);
        } else {
            return self.even.value(uv, p);
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    #[allow(unused_variables)]
    fn value(&self, uv: &TextureCoord, p: &Point3) -> Color {
        return Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turb(p, 7)));
    }
}

pub struct ImageTexture {
    buffer: Buffer,
    width: u32,
    height: u32,
}

impl ImageTexture {
    /// Create a new ImageTexture from a PNG image
    pub fn new(path: String) -> ImageTexture {
        let buffer = Buffer::new_from_png(path);
        let width = buffer.width();
        let height = buffer.height();

        ImageTexture {
            buffer,
            width,
            height,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: &TextureCoord, _p: &Point3) -> Color {
        let x = (uv.u * self.width as f64) as u32;
        let y = (uv.v * self.height as f64) as u32;

        self.buffer.at(x, y)
    }
}
