
use image::Rgb;

/// Represents RGB colors
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    /// Construct a new Color struct
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    /// Construct a Color struct with all components set to 0.0
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    /// Convert to 8-bit Rgb struct from `image` crate
    pub fn to_image_color(&self) -> Rgb<u8> {
        Rgb([
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
        ])
    }
}
