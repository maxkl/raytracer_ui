
use cgmath::{Point3, InnerSpace};

use crate::color::Color;
use crate::ray::Ray;

/// Implement for objects that a ray can intersect with
pub trait Intersectable {
    /// Cast a ray at the object. Returns true if it hits
    fn intersect(&self, ray: &Ray) -> bool;
}

/// A sphere
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub color: Color,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // TODO: prettier, time render, multiple objects, lighting
        // Calculate vector from ray origin to sphere center (hypotenuse)
        let to_center = self.center - ray.origin;

        // Project to_center onto ray direction vector to get length of adjacent side
        let adjacent = to_center.dot(ray.direction);
        // The length of the hypotenuse is just he magnitude of the vector connecting the ray origin and the sphere center
        let hypotenuse_squared = to_center.magnitude2();
        // Length of opposite side (pythagorean theorem)
        let opposite_squared = hypotenuse_squared - adjacent.powi(2);

        // The opposite side is the smallest distance between the ray and the sphere center
        // Compare the opposite side and the sphere radius to determine whether the ray goes through the sphere
        let radius_squared = self.radius.powi(2);
        opposite_squared < radius_squared
    }
}

impl Sphere {
    /// Construct a sphere
    pub fn new(center: Point3<f32>, radius: f32, color: Color) -> Sphere {
        Sphere { center, radius, color }
    }
}
