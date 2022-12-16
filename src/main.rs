use std::io;
use std::io::Write;
mod vec3;
use vec3::Color;
mod color;
use color::*;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height-1).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            // Classic graphics gradient
            let pixel_color = Color {
                e: [(i as f64) /((image_width-1) as f64), (j as f64)/((image_height-1) as f64), 0.25]
            };
            write_color(pixel_color);
        }
    }

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!()
}
