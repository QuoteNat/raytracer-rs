use crate::vec3::Color;

pub fn write_color(pixel_color: Color) {
    print!("{} {} {}\n", (pixel_color.x()* 255.99) as i32, 
    (pixel_color.y() * 255.99) as i32, 
    (pixel_color.z() * 255.99) as i32)
}