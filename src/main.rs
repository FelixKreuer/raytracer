use image::RgbImage;
#[allow(dead_code)]
mod maths;
use maths::vector3::Point3;
use maths::vector3::Vector3;
use maths::ray::Ray;    
mod utils;
use utils::color::write_color;
use utils::color::Color;
use utils::progressbar::visualise_progress;

fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = Vector3::dot(&r.dir, &r.dir);
    let b = 2.0 * Vector3::dot(&oc, &r.dir);
    let c = Vector3::dot(&oc, &oc) - radius * radius;
    let discriminant = b*b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt() ) / (2.0*a);
    }
}


fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vector3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }
    let unit_direction = r.dir.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
}

fn main() {
    // Hallo XYZ
    // Image
    // test
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: u32 = 1920;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = ((image_width as f32) / aspect_ratio) as u32;
    let image_height = image_height.max(1);

    // Camera

    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * (image_width as f32) / (image_height as f32);
    let camera_origin = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_origin
                             - Vector3::new(0.0, 0.0, focal_length)
                             - viewport_u / 2.0
                             - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    

    // Specify the output file path
    let output_file_path = "output/image.png";

    // Create or overwrite the file
    let mut img = RgbImage::new(image_width as u32, image_height as u32);

    for j in 0..image_height {
        visualise_progress(j, image_height);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_origin;
            let r = Ray::new(camera_origin, ray_direction);

            let pixel_color = ray_color(&r);
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

    img.save(output_file_path)
        .expect("Unable to save the image");
    println!("Image written to {}", output_file_path);
}
