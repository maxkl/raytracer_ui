
use serde::{Serialize, Deserialize};

use crate::color::Color;

/// Data struct collecting various material properties
#[derive(Serialize, Deserialize)]
pub struct Material {
    pub color: Color,
    pub albedo: f32,
}
