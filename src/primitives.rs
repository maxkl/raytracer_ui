
use cgmath::{Point3, InnerSpace};

use crate::color::Color;
use crate::ray::{Ray, Hit};

/// Implement for objects that a ray can intersect with
pub trait Intersectable {
    /// Cast a ray at the object. Returns true if it hits
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
}

/// A sphere
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub color: Color,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // Calculate vector from ray origin to sphere center (hypotenuse)
        let to_center = self.center - ray.origin;

        // Project to_center onto ray direction vector to get length of adjacent side
        let adjacent = to_center.dot(ray.direction);

        // Is the sphere behind the ray origin?
        if adjacent < 0.0 {
            return None;
        }

        // The length of the hypotenuse is just he magnitude of the vector connecting the ray origin and the sphere center
        let center_distance_squared = to_center.magnitude2();
        // Length of opposite side (pythagorean theorem)
        let distance_squared = center_distance_squared - adjacent.powi(2);

        // The opposite side is the smallest distance between the ray and the sphere center
        // Compare the opposite side and the sphere radius to determine whether the ray goes through the sphere
        let radius_squared = self.radius.powi(2);
        if distance_squared > radius_squared {
            return None;
        }

        // Calculate how thick the sphere is at the intersection point
        let thickness_half = (radius_squared - distance_squared).sqrt();
        // Calculate the distance along the ray of the two intersection points (front and back)
        let t0 = adjacent - thickness_half;
        let t1 = adjacent + thickness_half;

        // If both intersection points are behind us, return
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        // Choose the intersection point that is closer to the ray origin
        let distance = if t0 < t1 { t0 } else { t1 };

        Some(Hit::new(distance, self.color))
    }
}

impl Sphere {
    /// Construct a sphere
    pub fn new(center: Point3<f32>, radius: f32, color: Color) -> Sphere {
        Sphere { center, radius, color }
    }
}
