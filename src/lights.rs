
use std::f32;

use cgmath::{Vector3, Point3};
use serde::{Serialize, Deserialize};

use crate::color::Color;

#[typetag::serde(tag = "type")]
pub trait Light {
    fn direction_from(&self, point: &Point3<f32>) -> Vector3<f32>;
    fn color(&self, ) -> Color;
    fn intensity_at(&self, point: &Point3<f32>) -> f32;
    fn distance_at(&self, point: &Point3<f32>) -> f32;
}

/// A light that only has a direction, e.g. from the sun
#[derive(Serialize, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vector3<f32>,
    pub color: Color,
    pub intensity: f32,
}

#[typetag::serde]
impl Light for DirectionalLight {
    #[allow(unused_variables)]
    fn direction_from(&self, point: &Point3<f32>) -> Vector3<f32> {
        -self.direction
    }

    fn color(&self) -> Color {
        self.color
    }

    #[allow(unused_variables)]
    fn intensity_at(&self, point: &Point3<f32>) -> f32 {
        self.intensity
    }

    #[allow(unused_variables)]
    fn distance_at(&self, point: &Point3<f32>) -> f32 {
        f32::INFINITY
    }
}
