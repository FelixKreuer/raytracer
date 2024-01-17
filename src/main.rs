use image::RgbImage;
#[allow(dead_code)]
mod maths;
use maths::vector3::Vector3;
mod utils;
use utils::progressbar::visualise_progress;
use utils::color::Color;
use utils::color::write_color;
fn main() {
    // Image
    let image_width: u32 = 1024;
    let image_height: u32 = 1024;

    let v1 = Vector3::new(1.0, 1.0, 1.0);
    println!("{v1}");


    // Specify the output file path
    let output_file_path = "output/image.png";

    // Create or overwrite the file
    let mut img = RgbImage::new(image_width as u32, image_height as u32);

    for j in 0..image_height {
        visualise_progress(j, image_height);
        for i in 0..image_width {
            let pixel_color = Color::new(i as f32/(image_width-1)as f32 , j as f32/(image_height-1) as f32, 0f32);
            write_color(&mut img, pixel_color, i, j);
            /*
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            // Set the pixel color in the image buffer
            img.put_pixel(i, j, Rgb([ir, ig, ib]));*/
        }
    }
    println!("\r Finished rendering. {image_height}/{image_height} Lines");

    img.save(output_file_path).expect("Unable to save the image");
    println!("Image written to {}", output_file_path);
}