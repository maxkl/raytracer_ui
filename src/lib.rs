
mod color;
mod ray;
mod primitives;
mod lights;
mod scene;

use std::io::ErrorKind;
use std::time::Instant;
use std::f32;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use cgmath::{Vector3, Point3, InnerSpace};

use crate::color::Color;
use crate::ray::Ray;
use crate::primitives::{Sphere, Plane};
use crate::lights::DirectionalLight;
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
            let color = if let Some(hit) = hit {
                // Vector that points towards the light
                let to_light = -scene.light.direction;

                // Cast ray towards the light to check whether the point lies in the shadow
                let shadow_ray = Ray { origin: hit.point, direction: to_light };
                let in_light = scene.trace(&shadow_ray).is_none();

                if in_light {
                    // Calculate color using Lambert's Cosine Law
                    let light_power = hit.normal.dot(to_light).max(0.0) * scene.light.intensity;
                    let reflection_factor = hit.albedo / f32::consts::PI;
                    let color = hit.color * scene.light.color * light_power * reflection_factor;
                    // Ensure that color components are between 0.0 and 1.0
                    color.clamp()
                } else {
                    Color::black()
                }
            } else {
                scene.clear_color
            };
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
            Box::new(Plane::new(Point3::new(0.0, -2.0, 0.0), Vector3::new(0.0, 1.0, 0.0), Color::new(0.2, 0.2, 0.2), 0.18)),
            Box::new(Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.2, 1.0, 0.2), 0.18)),
            Box::new(Sphere::new(Point3::new(-3.0, 1.0, -6.0), 2.0, Color::new(0.2, 0.2, 1.0), 0.18)),
            Box::new(Sphere::new(Point3::new(2.0, 1.0, -4.0), 1.5, Color::new(1.0, 0.2, 0.2), 0.18)),
        ],
        light: DirectionalLight {
            direction: Vector3::new(-0.3, -1.0, -0.4).normalize(),
            color: Color::new(1.0, 1.0, 1.0),
            intensity: 20.0,
        },
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