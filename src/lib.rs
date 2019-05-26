
mod math_util;
mod color;
mod material;
mod ray;
mod primitives;
mod lights;
mod scene;
mod renderer;

use std::io::ErrorKind;
use std::time::Instant;

use crate::scene::Scene;

/// Render the scene and store the resulting image at `output_file_name`
pub fn main(scene_file_name: &str, output_file_name: &str) -> i32 {
    // Load scene from scene definition file
    let scene = Scene::load(scene_file_name).unwrap();

    let now = Instant::now();

    // Render scene
    let img = renderer::render(&scene);

    let duration = now.elapsed();
    println!("Rendered scene in {:.3} ms", duration.as_micros() as f64 * 1e-3);

    // Save image
    if let Err(err) = img.save(output_file_name) {
        match err.kind() {
            ErrorKind::InvalidInput => {
                eprintln!("Error: invalid file extension");
                return 1;
            }
            _ => {}
        }
    }

    0
}