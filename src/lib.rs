
use std::io::ErrorKind;

use image::{DynamicImage, GenericImage, GenericImageView, Rgb, Pixel};
use cgmath::{Point3, Vector3, InnerSpace};

struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    fn calculate(x: u32, y: u32, width: u32, height: u32, fov: f32) -> Ray {
        let fov_adjustment = (fov.to_radians() / 2.0).tan();
        let aspect_ratio = width as f32 / height as f32;
        let sensor_x = (((x as f32 + 0.5) / width as f32) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = 1.0 - ((y as f32 + 0.5) / height as f32) * 2.0 * fov_adjustment;

        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }
}

trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

#[derive(Copy, Clone)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn to_image_color(&self) -> Rgb<u8> {
        Rgb([
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ])
    }
}

struct Sphere {
    center: Point3<f32>,
    radius: f32,
    color: Color
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        let l = self.center - ray.origin;
        let adj2 = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj2 * adj2);
        d2 < (self.radius * self.radius)
    }
}

impl Sphere {
    fn new(center: Point3<f32>, radius: f32, color: Color) -> Sphere {
        Sphere { center, radius, color }
    }
}

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