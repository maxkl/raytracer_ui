
mod color;
mod ray;
mod primitives;
mod scene;

use std::io::ErrorKind;
use std::time::Instant;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use cgmath::Point3;

use crate::color::Color;
use crate::ray::Ray;
use crate::primitives::Sphere;
use crate::scene::Scene;

/// Render the scene to a new image
fn render_scene(scene: &Scene) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(800, 600);

    // Iterate over the entire image pixel by pixel
    let w = img.width();
    let h = img.height();
    for y in 0..h {
        for x in 0..w {
            // Construct ray
            let ray = Ray::from_screen_coordinates(x, y, w, h, 90.0);
            // Calculate intersection
            let hit = scene.trace(&ray);
            // Assign appropriate color
            let color = hit.map_or(scene.clear_color, |hit| hit.color);
            // Assign pixel value
            img.put_pixel(x, y, color.to_image_color().to_rgba());
        }
    }

    img
}

/// Render the scene and store the resulting image at `output_file_name`
pub fn main(output_file_name: &str) -> i32 {
    let scene = Scene {
        clear_color: Color::new(0.6, 0.8, 1.0),
        objects: vec![
            Box::new(Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.2, 1.0, 0.2))),
            Box::new(Sphere::new(Point3::new(-3.0, 1.0, -6.0), 2.0, Color::new(0.2, 0.2, 1.0))),
            Box::new(Sphere::new(Point3::new(2.0, 1.0, -4.0), 1.5, Color::new(1.0, 0.2, 0.2))),
        ]
    };

    let now = Instant::now();

    // Render scene
    let img = render_scene(&scene);

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