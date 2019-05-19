
use cgmath::Vector3;
use serde::{Serialize, Deserialize};

use crate::color::Color;

/// A light that only has a direction, e.g. from the sun
#[derive(Serialize, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vector3<f32>,
    pub color: Color,
    pub intensity: f32,
}
