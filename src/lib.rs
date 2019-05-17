
mod color;
mod ray;
mod primitives;

use std::io::ErrorKind;
use std::time::Instant;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use cgmath::Point3;

use crate::color::Color;
use crate::ray::Ray;
use crate::primitives::{Intersectable, Sphere};

/// Render the scene to a new image
fn render_scene() -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(800, 600);

    // Define the objects in the scene
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.4, 1.0, 0.4));

    // Iterate over the entire image pixel by pixel
    let w = img.width();
    let h = img.height();
    for y in 0..h {
        for x in 0..w {
            // Construct ray
            let ray = Ray::from_screen_coordinates(x, y, w, h, 90.0);
            // Calculate intersection
            let hit = sphere.intersect(&ray);
            // Assign appropriate color
            let color = if hit {
                sphere.color
            } else {
                Color::black()
            };
            // Assign pixel value
            img.put_pixel(x, y, color.to_image_color().to_rgba());
        }
    }

    img
}

/// Render the scene and store the resulting image at `output_file_name`
pub fn main(output_file_name: &str) -> i32 {
    let now = Instant::now();

    // Render scene
    let img = render_scene();

    let duration = now.elapsed();
    println!("Rendered scene in {:.3} ms", duration.as_micros() as f64 * 1e-3);

    // Save image
    if let Err(err) = img.save(output_file_name) {
        match err.kind() {
            ErrorKind::InvalidInput => {
                eprintln!("Error: invalid file extension");
                return 1;
            }
            _ => {}
        }
    }

    0
}