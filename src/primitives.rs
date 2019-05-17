
use cgmath::{Point3, InnerSpace};

use crate::color::Color;
use crate::ray::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
    pub color: Color,
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
    pub fn new(center: Point3<f32>, radius: f32, color: Color) -> Sphere {
        Sphere { center, radius, color }
    }
}
