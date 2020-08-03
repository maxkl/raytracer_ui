
use std::{env, process, fs};
use std::time::Instant;

use raytracer::{Scene, Renderer};
use std::error::Error;

/// Load a scene from a scene definition file in RON format
pub fn load_scene(scene_file_name: &str) -> Result<Scene, Box<dyn Error>> {
    // Load file as string
    let source = fs::read_to_string(scene_file_name)?;

    // Deserialize scene from string
    let scene = ron::de::from_str(&source)?;

    Ok(scene)
}

/// Render the scene and store the resulting image at `output_file_name`
pub fn render_scene_file(scene_file_name: &str, output_file_name: &str) -> Result<(), Box<dyn Error>> {
    let scene = load_scene(scene_file_name)?;

    let now = Instant::now();

    let renderer = Renderer::new(scene);
    let img = renderer.render();

    let duration = now.elapsed();
    println!("Rendered scene in {:.3} ms", duration.as_micros() as f64 * 1e-3);

    img.save(output_file_name)?;

    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 3 {
        eprintln!("Error: incorrect number of arguments");
        eprintln!("Usage: {} <scene file name> <output file name>", args[0]);
        process::exit(1);
    }

    let scene_file_name = &args[1];
    let output_file_name = &args[2];

    if let Err(err) = render_scene_file(scene_file_name, output_file_name) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
