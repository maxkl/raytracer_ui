
use std::error::Error;
use std::{env, process, fs, thread};
use std::time::Duration;
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::collections::VecDeque;

use image::{DynamicImage, GenericImageView, Pixel, GenericImage};
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

struct RenderArea {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

struct RenderResult {
    image: DynamicImage,
    x: u32,
    y: u32,
}

fn gen_chunks(w: u32, h: u32, chunk_size: u32) -> VecDeque<RenderArea> {
    let mut chunks = VecDeque::new();
    for y in (0..h).step_by(chunk_size as usize) {
        for x in (0..w).step_by(chunk_size as usize) {
            chunks.push_back(RenderArea {
                x,
                y,
                w: (w - x).min(chunk_size),
                h: (h - y).min(chunk_size)
            });
        }
    }

    chunks
}

struct RenderThread {
    join_handle: JoinHandle<()>,
    tx: Sender<RenderArea>,
    rx: Receiver<RenderResult>,
}

impl RenderThread {
    fn run(scene: Scene, rx: Receiver<RenderArea>, tx: Sender<RenderResult>) {
        let renderer = Renderer::new(scene);

        loop {
            match rx.recv() {
                Ok(area) => {
                    let result = renderer.render_rect(area.x, area.y, area.w, area.h);

                    tx.send(RenderResult {
                        image: result,
                        x: area.x,
                        y: area.y
                    }).unwrap();
                }
                Err(_) => break
            }
        }
    }

    fn start(scene: Scene) -> RenderThread {
        let (join_handle, rx, tx) = {
            let (to_thread, from_main) = channel();
            let (to_main, from_thread) = channel();

            let join_handle = thread::spawn(move || RenderThread::run(scene, from_main, to_main));

            (join_handle, from_thread, to_thread)
        };

        RenderThread {
            join_handle,
            tx,
            rx,
        }
    }
}

fn assign_chunk(render_thread: &RenderThread, chunks: &mut VecDeque<RenderArea>) {
    if let Some(chunk) = chunks.pop_front() {
        render_thread.tx.send(chunk).unwrap();
    }
}

fn render_loop(scene: &Scene) -> Result<(), Box<dyn Error>> {
    let thread_count = num_cpus::get();

    println!("Using {} threads", thread_count);

    let render_threads: Vec<_> = (0..thread_count).map(|_| RenderThread::start(scene.clone())).collect();

    let width = scene.image_size.0;
    let height = scene.image_size.1;

    let mut window = Window::new(
        "Render result - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default()
    ).expect("Failed to create window");
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; width as usize * height as usize];
    let mut image = DynamicImage::new_rgb8(width, height);

    let mut image_changed = false;

    let mut chunks = gen_chunks(width, height, 100);

    for render_thread in &render_threads {
        assign_chunk(render_thread, &mut chunks);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for render_thread in &render_threads {
            match render_thread.rx.try_recv() {
                Ok(result) => {
                    image.copy_from(&result.image, result.x, result.y).unwrap();
                    image_changed = true;

                    assign_chunk(render_thread, &mut chunks);
                },
                Err(TryRecvError::Empty) => { /* No new message */ },
                Err(TryRecvError::Disconnected) => panic!("Render thread stopped unexpectedly"),
            };
        }

        if image_changed {
            image_changed = false;

            for (i, (_x, _y, color)) in image.pixels().enumerate() {
                let channels = color.channels();
                buffer[i] = ((channels[0] as u32) << 16) | ((channels[1] as u32) << 8) | channels[2] as u32;
            }
        }

        window.update_with_buffer(&buffer, width as usize, height as usize).unwrap();
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

    let scene = load_scene(&scene_file_name).unwrap();

    if let Err(err) = render_loop(&scene) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
