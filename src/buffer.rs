use crate::utility::clamp;
use crate::vector::{zero_vec, Color};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Buffer struct for storing image color data
pub struct Buffer {
    e: Vec<Color>,
    width: u32,
    height: u32,
}

impl Buffer {
    pub fn new(width: u32, height: u32) -> Buffer {
        Buffer {
            // Vec length is width * height of the image * 3 (for the r, g, and b values of each pixel)
            e: vec![zero_vec(); (width * height) as usize],
            width,
            height,
        }
    }

    /// Returns the 1d array index of [r(ow), c(olumn)]
    fn index(&self, r: u32, c: u32) -> usize {
        return ((self.height - r - 1) * self.width + c) as usize;
    }

    /// Change value at [x, y] to value
    pub fn write(&mut self, value: Color, x: u32, y: u32) {
        let index = self.index(y, x);
        // Write each color value separately into the array
        self.e[index] = value;
    }

    /// Returns value at [x, y]
    // pub fn at(&self, x: u32, y: u32) -> Color {
    //     let index = self.index(y, x);

    //     Color {
    //         e: [self.e[index], self.e[index + 1], self.e[index + 2]],
    //     }
    // }

    /// Outputs the buffer to a png file
    pub fn buffer_to_png(&self, path: String) {
        let path = Path::new(&path);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));

        let mut writer = encoder.write_header().unwrap();

        // Copy buffer as i32
        let mut data: Vec<u8> = Vec::new();
        for element in &self.e {
            data.push((256.0 * clamp(element.x(), 0.0, 0.999)) as u8);
            data.push((256.0 * clamp(element.y(), 0.0, 0.999)) as u8);
            data.push((256.0 * clamp(element.z(), 0.0, 0.999)) as u8);
        }

        writer.write_image_data(&data).unwrap();
    }
}
