
use cgmath::{Point3, Vector3, InnerSpace};

/// Represents a single ray with origin and direction
pub struct Ray {
    /// Ray origin
    pub origin: Point3<f32>,
    /// Unit vector representing the rays direction
    pub direction: Vector3<f32>,
}

impl Ray {
    /// Create a ray with the appropriate direction for the specified pixel position and field of view
    pub fn from_screen_coordinates(x: u32, y: u32, width: u32, height: u32, fov: f32) -> Ray {
        let fov_factor = (fov.to_radians() / 2.0).tan();

        let aspect_ratio = width as f32 / height as f32;

        // Calculate screen coordinates between 0 and 1
        let x_01 = (x as f32 + 0.5) / width as f32;
        let y_01 = (y as f32 + 0.5) / height as f32;

        // Translate screen coordinates in range [0.0, 1.0] to range [-1.0, 1.0]
        let x_relative = x_01 * 2.0 - 1.0;
        let y_relative = -(y_01 * 2.0 - 1.0);

        // Calculate ray direction from screen coordinates
        let ray_x = x_relative * aspect_ratio * fov_factor;
        let ray_y = y_relative * fov_factor;

        let direction_normalized = Vector3::new(ray_x, ray_y, -1.0).normalize();

        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: direction_normalized,
        }
    }
}
