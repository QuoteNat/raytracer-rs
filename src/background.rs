use crate::vector::{Vec3, Color, unit_vector};

pub trait Background {
    /// Returns the background color for a given ray direction
    fn apply(&self, dir: Vec3) -> Color;
}

/// A background that is always a single consistent color
pub struct BackgroundColor {
    color: Color
}

impl BackgroundColor {
    pub fn new(color: Color) -> BackgroundColor {
        BackgroundColor { color }
    }
}

impl Background for BackgroundColor {
    #[allow(unused_variables)]
    fn apply(&self, dir: Vec3) -> Color {
        self.color
    }
}

pub struct GradientY {
    color1: Color,
    color2: Color,
}

impl GradientY {
    pub fn new(color1: Color, color2: Color) -> GradientY {
        GradientY { color1, color2 }
    }
}

impl Background for GradientY {
    fn apply(&self, dir: Vec3) -> Color {
        let unit_direction = unit_vector(dir);
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0-t) * self.color1 + t * self.color2;
    }
}
