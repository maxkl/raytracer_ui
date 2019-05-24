
use cgmath::{Point3, InnerSpace, Vector3};
use serde::{Serialize, Deserialize};

use crate::material::Material;
use crate::ray::{Ray, Hit, Intersectable};
use crate::math_util::deserialize_normalized;

/// A plane
#[derive(Serialize, Deserialize)]
pub struct Plane {
    pub p0: Point3<f32>,
    #[serde(deserialize_with = "deserialize_normalized")]
    pub normal: Vector3<f32>,
    pub material: Material,
}

#[typetag::serde]
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // The normal has to be inverted for this calculation
        let normal = -self.normal;

        // Calculate intersection
        let denominator = normal.dot(ray.direction);
        if denominator > 0.0 {
            let to_p0 = self.p0 - ray.origin;
            let distance = to_p0.dot(normal) / denominator;
            if distance > 0.0 {
                let hit_point = ray.origin + distance * ray.direction;
                return Some(Hit::new(hit_point, distance, self.normal, &self.material))
            }
        }

        None
    }
}

/// A sphere
#[derive(Serialize, Deserialize)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub material: Material,
}

#[typetag::serde]
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

        let hit_point = ray.origin + distance * ray.direction;
        let normal = (hit_point - self.center).normalize();

        Some(Hit::new(hit_point, distance, normal, &self.material))
    }
}
