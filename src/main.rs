mod lights;
mod materials;
mod primitives;
mod renderer;
mod scenes;

use anyhow::{anyhow, Context, Error};
use argparse::{ArgumentParser, Parse, Print, Store};
use image::save_buffer;
use renderer::{Renderer, SuperSamplingMode};

use scenes::scene_lookup;
use std::path::PathBuf;
use std::time::Instant;

fn main() -> Result<(), Error> {
    let mut image_path = PathBuf::new();
    let mut scene_name = String::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut fov: f64 = 0.0;
    let mut workers = num_cpus::get();
    let mut samples_arg: u8 = 1;

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("A raytracer written in Rust");
        parser.add_option(
            &["-v", "--version"],
            Print(format!("rayblaster: v{}", env!("CARGO_PKG_VERSION"))),
            "Show version",
        );
        parser
            .refer(&mut image_path)
            .add_option(&["-o", "--output"], Parse, "Place the output into <file>")
            .required();
        parser
            .refer(&mut scene_name)
            .add_option(&["-s", "--scene"], Store, "The scene to render")
            .required();
        parser
            .refer(&mut width)
            .add_option(&["-w", "--width"], Store, "The width of the output image")
            .required();
        parser
            .refer(&mut height)
            .add_option(&["-h", "--height"], Store, "The height of the output image")
            .required();
        parser
            .refer(&mut fov)
            .add_option(&["-f", "--fov"], Store, "The fov of the output image")
            .required();
        parser.refer(&mut workers).add_option(
            &["-t", "--threads"],
            Store,
            "The number of worker threads to spawn",
        );
        parser.refer(&mut samples_arg).add_option(
            &["--samples"],
            Store,
            "The number of samples per pixel",
        );
        parser.parse_args_or_exit();
    }

    let mut buffer = vec![0; width * height * 3].into_boxed_slice();

    let scene =
        scene_lookup(&scene_name).with_context(|| anyhow!("No primitives in {}", scene_name))?;

    if scene.primitives.is_empty() {
        return Err(anyhow!("Empty scene: {}", scene_name));
    }

    let samples = match samples_arg {
        1 => SuperSamplingMode::SSAAX1,
        4 => SuperSamplingMode::SSAAX4,
        16 => SuperSamplingMode::SSAAX16,
        _ => return Err(anyhow!("Unsupported SSAA mode {}", samples_arg)),
    };

    let t = Instant::now();

    let renderer = Renderer::new(width, height, workers, scene, fov, samples);
    let frame = renderer.render();

    println!(
        "Rendered in {}ms",
        (t.elapsed().as_secs() * 1000) + t.elapsed().subsec_millis() as u64
    );

    for (i, pixel) in frame.iter().enumerate() {
        buffer[i * 3] = pixel.x;
        buffer[i * 3 + 1] = pixel.y;
        buffer[i * 3 + 2] = pixel.z;
    }
    save_buffer(
        image_path,
        &buffer,
        width as u32,
        height as u32,
        image::RGB(8),
    )
    .map_err(Error::from)
}
