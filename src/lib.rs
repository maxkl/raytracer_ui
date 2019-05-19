
mod color;
mod material;
mod ray;
mod primitives;
mod lights;
mod scene;

use std::io::ErrorKind;
use std::time::Instant;
use std::f32;
use std::fs;
use std::error::Error;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use cgmath::InnerSpace;

use crate::color::Color;
use crate::ray::Ray;
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
            let ray = Ray::from_screen_coordinates(x, y, w, h, 45.0);
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
                    let reflection_factor = hit.material.albedo / f32::consts::PI;
                    let color = hit.material.color * scene.light.color * light_power * reflection_factor;
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

/// Load a scene from a scene definition file in RON format
fn load_scene(scene_file_name: &str) -> Result<Scene, Box<dyn Error>> {
    // Load file as string
    let source = fs::read_to_string(scene_file_name)?;

    // Deserialize scene from string
    let scene = ron::de::from_str(&source)?;

    Ok(scene)
}

/// Render the scene and store the resulting image at `output_file_name`
pub fn main(scene_file_name: &str, output_file_name: &str) -> i32 {
    // Load scene from scene definition file
    let scene = load_scene(scene_file_name).unwrap();

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