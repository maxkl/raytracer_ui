
use std::error::Error;
use std::{env, process, fs, thread};
use std::time::{Duration, Instant};
use std::thread::JoinHandle;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::collections::VecDeque;
use std::path::Path;

use image::{DynamicImage, GenericImageView, Pixel, GenericImage};
use minifb::{Window, WindowOptions, Key};
use nfd::Response;
use serde::{Serialize, Deserialize};

use raytracer::{Renderer, Scene, RgbImage, asset_loader, ObjParser, MeshData};
use raytracer::asset_loader::AssetLoader;

#[derive(Clone, Serialize, Deserialize)]
pub struct AssetLoaderImpl {}

impl AssetLoader for AssetLoaderImpl {
    fn load_image(&self, path: &Path) -> Result<RgbImage, Box<dyn Error>> {
        let img = image::open(&path)?;
        let w = img.width();
        let h = img.height();
        let data = img.into_rgb().into_raw();
        Ok(RgbImage::from_raw(w as usize, h as usize, data))
    }

    fn load_obj(&self, path: &Path) -> Result<MeshData, Box<dyn Error>> {
        let obj_str = fs::read_to_string(path)?;

        let mesh = ObjParser::parse(&obj_str)?;

        Ok(mesh)
    }
}

impl AssetLoaderImpl {
    fn new() -> AssetLoaderImpl {
        AssetLoaderImpl {}
    }
}

/// Load a scene from a scene definition file in RON format
pub fn load_scene(scene_file_name: &str) -> Result<Scene, Box<dyn Error>> {
    // Load file as string
    let source = fs::read_to_string(scene_file_name)?;

    // Deserialize scene from string
    let scene = ron::de::from_str(&source)?;

    Ok(scene)
}

struct RenderArea {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

struct RenderResult {
    image: RgbImage,
    x: usize,
    y: usize,
}

fn gen_chunks(w: usize, h: usize, chunk_size: usize) -> VecDeque<RenderArea> {
    let mut chunks = VecDeque::new();
    for y in (0..h).step_by(chunk_size) {
        for x in (0..w).step_by(chunk_size) {
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

    let image_size = scene.camera.resolution;
    let width = image_size.0;
    let height = image_size.1;

    let mut window = Window::new(
        "Render result - S to save, ESC to exit",
        width,
        height,
        WindowOptions::default()
    ).expect("Failed to create window");
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut image = DynamicImage::new_rgb8(width as u32, height as u32);

    let mut image_changed = false;

    let mut chunks = gen_chunks(width, height, 100);
    let chunks_total = chunks.len();
    let mut chunks_completed = 0;

    let start_time = Instant::now();

    for render_thread in &render_threads {
        assign_chunk(render_thread, &mut chunks);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for render_thread in &render_threads {
            match render_thread.rx.try_recv() {
                Ok(result) => {
                    let RenderResult {
                        image: result_image,
                        x: result_x,
                        y: result_y
                    } = result;

                    let w = result_image.width();
                    let h = result_image.height();
                    let data = result_image.into_raw();

                    let img = DynamicImage::ImageRgb8(image::RgbImage::from_raw(w as u32, h as u32, data).unwrap());
                    image.copy_from(&img, result_x as u32, result_y as u32).unwrap();
                    image_changed = true;

                    chunks_completed += 1;
                    if chunks_completed == chunks_total {
                        let duration = start_time.elapsed();
                        println!("Render completed in {:.5} s", duration.as_secs_f64());
                    }

                    assign_chunk(render_thread, &mut chunks);
                },
                Err(TryRecvError::Empty) => { /* No new message */ },
                Err(TryRecvError::Disconnected) => panic!("Render thread stopped unexpectedly"),
            };
        }

        if chunks.is_empty() {
            // Rendering done

            if window.is_key_down(Key::S) {
                match nfd::open_save_dialog(Some("png,jpg,jpeg"), None).unwrap() {
                    Response::Okay(path) => {
                        image.save(path.clone()).unwrap();
                        println!("Image saved to {}", path);
                    },
                    Response::OkayMultiple(_) => unreachable!(),
                    Response::Cancel => {},
                }
            }
        }

        if image_changed {
            image_changed = false;

            for (i, (_x, _y, color)) in image.pixels().enumerate() {
                let channels = color.channels();
                buffer[i] = ((channels[0] as u32) << 16) | ((channels[1] as u32) << 8) | channels[2] as u32;
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
    }

    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Error: incorrect number of arguments");
        eprintln!("Usage: {} <scene file name>", args[0]);
        process::exit(1);
    }

    let scene_file_name = &args[1];

    asset_loader::set_instance(Box::new(AssetLoaderImpl::new()));

    let scene = load_scene(&scene_file_name).unwrap();

    if let Err(err) = render_loop(&scene) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
