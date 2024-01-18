use image::{Rgb, RgbImage};

use crate::maths::vector3::Vector3;
pub type Color = Vector3; // Assuming Vector3 is your color type

pub fn write_color(img: &mut RgbImage, pixel_color: Color, x_location: u32, y_location: u32) {
    // Write the translated [0,255] value of each color component.
    let ir = (255.999 * pixel_color.x) as u8;
    let ig = (255.999 * pixel_color.y) as u8;
    let ib = (255.999 * pixel_color.z) as u8;
    img.put_pixel(x_location, y_location, Rgb([ir, ig, ib]));
}
