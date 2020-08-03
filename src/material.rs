
use std::path::PathBuf;

use serde::{Serialize, Deserialize, Deserializer, Serializer};
use image::{DynamicImage, GenericImageView, Pixel, ImageError};

use crate::color::Color;
use crate::math_util::Modulo;

/// Generic texture/UV coordinates
#[derive(Copy, Clone)]
pub struct TexCoords<T> {
    pub u: T,
    pub v: T,
}

/// Represents a texture.
///
/// Serializes/deserializes to/from a string, which is the path to the image file
pub struct Texture {
    pub path: PathBuf,
    pub img: DynamicImage,
}

impl Serialize for Texture {
    /// Serialize this texture to a string, which is the image file path
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        // Serialize file path
        self.path.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Texture {
    /// Deserialize a texture from a string, which is the image file path
    fn deserialize<D>(deserializer: D) -> Result<Texture, D::Error>
    where
        D: Deserializer<'de>
    {
        // Deserialize file path
        let path = PathBuf::deserialize(deserializer)?;
        // Load texture image from path
        Self::load(path.clone()).map_err(|err| {
            serde::de::Error::custom(format!("Unable to open image file \"{}\": {}", path.display(), err))
        })
    }
}

impl Texture {
    /// Load a texture from an image file
    fn load(path: PathBuf) -> Result<Texture, ImageError> {
        let img = image::open(&path)?;
        Ok(Texture {
            path: path,
            img: img,
        })
    }
}

/// Represents the various ways a point can be colored
#[derive(Serialize, Deserialize)]
pub enum Coloration {
    /// Uniform color
    Color(Color),
    /// Get color for each point from a texture
    Texture(Texture),
}

impl Coloration {
    /// Calculate color at a specific position
    pub fn color(&self, tex_coords: &TexCoords<f32>) -> Color {
        match self {
            Coloration::Color(color) => *color,
            Coloration::Texture(tex) => {
                let tex_w = tex.img.width() as f32;
                let tex_h = tex.img.height() as f32;
                // Map UV coordinates to pixel coordinates and wrap when they exceed the image boundaries
                let tex_x = (tex_coords.u * tex_w).modulo(tex_w);
                let tex_y = (tex_coords.v * tex_h).modulo(tex_h);

                // Get color of pixel at the specified position
                Color::from_rgb(tex.img.get_pixel(tex_x as u32, tex_y as u32).to_rgb())
            }
        }
    }
}

/// Data struct collecting various material properties
#[derive(Serialize, Deserialize)]
pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub refractive_index: f32,
}
