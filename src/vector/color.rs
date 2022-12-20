use super::vec3::Color;
use super::super::utility::*;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    let r = r * scale;
    let g = g * scale;
    let b = b * scale;
    
    print!("{} {} {}\n", (256.0 * clamp(r, 0.0, 0.999)) as i32, 
        (256.0 * clamp(g, 0.0, 0.999)) as i32, 
        (256.0 * clamp(b, 0.0, 0.999)) as i32);
}