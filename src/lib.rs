
mod color;
mod ray;
mod primitives;

use std::io::ErrorKind;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use cgmath::Point3;

use crate::color::Color;
use crate::ray::Ray;
use crate::primitives::{Intersectable, Sphere};

fn render_scene() -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(800, 600);

    let sphere = Sphere::new(Point3::new(0.0, 0.0, -5.0), 1.0, Color::new(0.4, 1.0, 0.4));

    let w = img.width();
    let h = img.height();
    for y in 0..h {
        for x in 0..w {
            let ray = Ray::calculate(x, y, w, h, 90.0);
            let hit = sphere.intersect(&ray);
            let color = if hit {
                sphere.color
            } else {
                Color::black()
            };
            img.put_pixel(x, y, color.to_image_color().to_rgba());
        }
    }

    img
}

pub fn main(output_file_name: &str) -> i32 {
    let img = render_scene();

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