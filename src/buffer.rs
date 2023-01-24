use crate::utility::clamp;
use crate::vector::{zero_vec, Color, Vec3};
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

    pub fn new_from_png(path: String) -> Buffer {
        // read in the png file
        let decoder = png::Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = &buf[..info.buffer_size()];
        // fetch the width and height
        let header = reader.info();
        let width = header.width;
        let height = header.height;
        // allocate the Color buffer
        let mut e = vec![Vec3::new(0.0, 0.0, 0.0); (width * height) as usize];

        for i in (0..reader.output_buffer_size()).step_by(3) {
            e.push(Vec3::new(
                bytes[i] as f64,
                bytes[i + 1] as f64,
                bytes[i + 2] as f64,
            ))
        }

        Buffer { e, width, height }
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
    pub fn at(&self, x: u32, y: u32) -> Color {
        let index = self.index(y, x);

        return self.e[index];
    }

    /// Returns buffer width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns buffer height
    pub fn height(&self) -> u32 {
        self.height
    }

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
