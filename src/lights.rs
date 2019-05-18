
use cgmath::Vector3;

use crate::color::Color;

// A light that only has a direction, e.g. from the sun
pub struct DirectionalLight {
    pub direction: Vector3<f32>,
    pub color: Color,
    pub intensity: f32,
}
