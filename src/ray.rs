
use cgmath::{Point3, Vector3, InnerSpace};

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn calculate(x: u32, y: u32, width: u32, height: u32, fov: f32) -> Ray {
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
