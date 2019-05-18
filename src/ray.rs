
use cgmath::{Point3, Vector3, InnerSpace};
use std::cmp::Ordering;
use crate::color::Color;

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

pub struct Hit {
    pub point: Point3<f32>,
    pub distance: f32,
    pub normal: Vector3<f32>,
    pub color: Color,
    pub albedo: f32,
}

impl PartialEq for Hit {
    /// Hits are equal when their hit distances are equal
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Hit {}

impl PartialOrd for Hit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hit {
    /// Compare hits by their hit distance
    fn cmp(&self, other: &Self) -> Ordering {
        // Hit distances should never be NaN or Infinity
        self.distance.partial_cmp(&other.distance).unwrap()
    }
}

impl Hit {
    pub fn new(point: Point3<f32>, distance: f32, normal: Vector3<f32>, color: Color, albedo: f32) -> Hit {
        Hit { point, distance, normal, color, albedo }
    }
}

/// Implement for objects that a ray can intersect with
pub trait Intersectable {
    /// Cast a ray at the object. Returns true if it hits
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}
