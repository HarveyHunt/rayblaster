extern crate cgmath;
extern crate crossbeam;
extern crate image;
extern crate num_cpus;
extern crate argparse;

mod renderer;
mod primitives;
mod lights;
mod scenes;
mod materials;

use argparse::{ArgumentParser, Store, Print, Parse};
use std::path::PathBuf;
use std::time::Instant;
use scenes::{scene_lookup, Scene};
use renderer::Renderer;
use cgmath::Vector3;
use image::{save_buffer, ColorType};

fn main() {
    let scene: Scene;
    let mut image_path = PathBuf::new();
    let mut scene_name = String::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut fov: f64 = 0.0;
    let mut workers = num_cpus::get();

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("A raytracer written in Rust");
        parser.add_option(&["-v", "--version"],
                          Print(format!("rayblaster: v{}", env!("CARGO_PKG_VERSION"))),
                          "Show version");
        parser.refer(&mut image_path)
            .add_option(&["-o", "--output"], Parse, "Place the output into <file>")
            .required();
        parser.refer(&mut scene_name)
            .add_option(&["-s", "--scene"], Store, "The scene to render")
            .required();
        parser.refer(&mut width)
            .add_option(&["-w", "--width"], Store, "The width of the output image")
            .required();
        parser.refer(&mut height)
            .add_option(&["-h", "--height"], Store, "The height of the output image")
            .required();
        parser.refer(&mut fov)
            .add_option(&["-f", "--fov"], Store, "The fov of the output image")
            .required();
        parser.refer(&mut workers)
            .add_option(&["-t", "--threads"], Store, "The number of worker threads to spawn");
        parser.parse_args_or_exit();
    }

    let mut buffer = vec![0; width * height * 3].into_boxed_slice();

    match scene_lookup(&scene_name) {
        Ok(s) => scene = s,
        Err(_) => panic!("Failed to load scene {}", scene_name),
    }

    if scene.primitives.len() == 0 {
        panic!("No primitives in {}", scene_name);
    }

    let t = Instant::now();
    let renderer = Renderer::new(width, height, workers, scene, fov);
    let frame = renderer.render();

    println!("Rendered in {}ms",
             (t.elapsed().as_secs() * 1000) + (t.elapsed().subsec_nanos() / 1000000) as u64);

    for (i, pixel) in frame.iter().enumerate() {
        buffer[i * 3] = pixel.x;
        buffer[i * 3 + 1] = pixel.y;
        buffer[i * 3 + 2] = pixel.z;
    }

    save_buffer(image_path,
                &buffer,
                width as u32,
                height as u32,
                image::RGB(8));
}
