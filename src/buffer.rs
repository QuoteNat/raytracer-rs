use crate::vector::{zero_vec, Color};

/// Buffer struct for storing image color data
pub struct Buffer {
    e: Vec<Color>,
    width: u32,
    height: u32,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Buffer {
        Buffer {
            e: vec![zero_vec(); (width * height) as usize],
            width,
            height,
        }
    }

    /// Returns the 1d array index of [r(ow), c(olumn)]
    fn index(&self, r: u32, c: u32) -> usize {
        return (r * self.height + c) as usize;
    }

    /// Change value at [x, y] to value
    pub fn write(&mut self, value: Color, x: u32, y: u32) {
        let index = self.index(y, x);
        self.e[index] = value;
    }

    /// Returns value at [x, y]
    pub fn at(&self, x: u32, y: u32) -> Color {
        self.e[self.index(y, x)]
    }
}
