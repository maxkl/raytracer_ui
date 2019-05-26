
use std::f32;

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
pub fn render(scene: &Scene) -> DynamicImage {
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