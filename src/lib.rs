
mod math_util;
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
use crate::ray::{Ray, Hit};
use crate::scene::Scene;

fn shade_diffuse(scene: &Scene, hit: &Hit) -> Color {
    let mut color = Color::black();

    // Sum contributions by all light sources
    for light in scene.lights.iter() {
        // Vector that points towards the light
        let to_light = light.direction_from(&hit.point);

        // Cast ray towards the light to check whether the point lies in the shadow
        let shadow_ray = Ray { origin: hit.point, direction: to_light };
        let shadow_hit = scene.trace(&shadow_ray);
        // Is there any object in the direction of the light that is closer than the light source?
        let in_light = match shadow_hit {
            Some(shadow_hit) => shadow_hit.distance > light.distance_at(&hit.point),
            None => true,
        };

        if in_light {
            // Calculate color using Lambert's Cosine Law
            let light_power = hit.normal.dot(to_light).max(0.0) * light.intensity_at(&hit.point);
            let reflection_factor = hit.material.albedo / f32::consts::PI;
            let material_color = hit.material.color.color(&hit.tex_coords);
            color += material_color * light.color() * light_power * reflection_factor;
        }
    }

    // Ensure that color components are between 0.0 and 1.0
    color.clamp()
}

fn get_color(scene: &Scene, ray: &Ray, hit: &Hit, depth: u32) -> Color {
    let diffuse_color = shade_diffuse(scene, hit);

    let reflective_color = if hit.material.reflectivity > 0.0 {
        let reflection_ray = Ray::create_reflection(&hit.normal, &ray.direction, &hit.point);
        cast_ray(scene, &reflection_ray, depth + 1)
    } else {
        Color::black()
    };

    (diffuse_color * (1.0 - hit.material.reflectivity) + reflective_color * hit.material.reflectivity + scene.ambient_light_color).clamp()
}

fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    if depth > scene.max_recursion_depth {
        return Color::black();
    }

    scene.trace(ray)
        .map(|hit| get_color(scene, ray, &hit, depth))
        .unwrap_or(scene.clear_color)
}

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
            // Assign appropriate color
            let color = cast_ray(scene, &ray, 0);
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