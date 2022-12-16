use std::io;
use std::io::Write;
mod vec3;

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
            let r: f64 = (i as f64) / (image_width-1) as f64;
            let g: f64 = (j as f64) / (image_height-1) as f64;
            let b: f64 = 0.25;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;
            
            println!("{} {} {}\n", ir, ig, ib)
        }
    }

    // Make it so progress indicator doesn't end up before terminal prompt
    eprintln!()
}
