
use std::error::Error;
use std::{env, process, fs, thread};
use std::time::{Instant, Duration};
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};

use image::{DynamicImage, GenericImageView, Pixel};
use minifb::{Window, WindowOptions, Key};

use raytracer::{Scene, Renderer};

/// Load a scene from a scene definition file in RON format
pub fn load_scene(scene_file_name: &str) -> Result<Scene, Box<dyn Error>> {
    // Load file as string
    let source = fs::read_to_string(scene_file_name)?;

    // Deserialize scene from string
    let scene = ron::de::from_str(&source)?;

    Ok(scene)
}

enum Cmd {
    Load(String),
    Render,
    Save(String),
}

struct ImageProperties {
    width: usize,
    height: usize,
}

enum CmdResult {
    Loaded(ImageProperties),
    Rendered(DynamicImage),
    Saved,
}

fn render_thread(rx: Receiver<Cmd>, tx: Sender<CmdResult>) {
    let mut renderer = None;
    let mut img = None;

    loop {
        match rx.recv() {
            Ok(cmd) => match cmd {
                Cmd::Load(scene_file_name) => {
                    let scene = load_scene(&scene_file_name).unwrap();

                    renderer = Some(Renderer::new(scene));

                    tx.send(CmdResult::Loaded(ImageProperties {
                        width: 800,
                        height: 600,
                    })).unwrap();
                }
                Cmd::Render => {
                    let renderer = renderer.as_ref().unwrap();

                    let now = Instant::now();

                    let result = renderer.render();

                    let duration = now.elapsed();
                    println!("Rendered scene in {:.3} ms", duration.as_micros() as f64 * 1e-3);

                    img = Some(result.clone());

                    tx.send(CmdResult::Rendered(result)).unwrap();
                }
                Cmd::Save(output_file_name) => {
                    let img = img.as_ref().unwrap();

                    img.save(&output_file_name).unwrap();

                    tx.send(CmdResult::Saved).unwrap();
                }
            }
            Err(_) => break
        }
    }
}

fn render_loop(scene_file_name: &str, output_file_name: &str) -> Result<(), Box<dyn Error>> {
    let (rx, tx) = {
        let (to_thread, from_main) = channel();
        let (to_main, from_thread) = channel();

        thread::spawn(move || render_thread(from_main, to_main));

        (from_thread, to_thread)
    };

    let mut window = None;

    let mut buffer: Vec<u32> = Vec::new();
    let mut image = None;

    let mut image_changed = false;

    tx.send(Cmd::Load(scene_file_name.to_string())).unwrap();

    let mut stop = false;

    while !stop {
        let result = match window {
            Some(_) => match rx.try_recv() {
                Ok(result) => Some(result),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => panic!("Render thread stopped unexpectedly"),
            }
            None => match rx.recv() {
                Ok(result) => Some(result),
                Err(_) => panic!("Render thread stopped unexpectedly"),
            }
        };

        if let Some(result) = result {
            match result {
                CmdResult::Loaded(image_properties) => {
                    image = Some(DynamicImage::new_rgb8(image_properties.width as u32, image_properties.height as u32));
                    image_changed = true;

                    let mut new_window = Window::new(
                        "Render result - ESC to exit",
                        image_properties.width,
                        image_properties.height,
                        WindowOptions::default()
                    ).expect("Failed to create window");

                    new_window.limit_update_rate(Some(Duration::from_micros(16600)));

                    window = Some(new_window);

                    tx.send(Cmd::Render).unwrap();
                }
                CmdResult::Rendered(img) => {
                    image = Some(img);
                    image_changed = true;
                }
                CmdResult::Saved => println!("Saved!")
            }
        }

        if let Some(window) = &mut window {
            if image_changed {
                image_changed = false;

                let img = image.as_ref().unwrap();
                let width = img.width() as usize;
                let height = img.height() as usize;

                buffer.resize(width * height, 0);
                for (i, (_x, _y, color)) in img.pixels().enumerate() {
                    let channels = color.channels();
                    buffer[i] = ((channels[0] as u32) << 16) | ((channels[1] as u32) << 8) | channels[2] as u32;
                }

                window.update_with_buffer(&buffer, width, height).unwrap();
            } else {
                window.update();
            }

            if !window.is_open() || window.is_key_down(Key::Escape) {
                stop = true;
            }
        }
    }

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

    if let Err(err) = render_loop(scene_file_name, output_file_name) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
