mod lights;
mod materials;
mod primitives;
mod renderer;
mod scenes;

use anyhow::{anyhow, Context, Error};
use image::save_buffer;
use renderer::{Renderer, SuperSamplingMode};
use structopt::StructOpt;

use scenes::scene_lookup;
use std::path::PathBuf;
use std::time::Instant;

#[derive(StructOpt, Debug)]
#[structopt(name = "rayblaster")]
/// A raytracer written in Rust
struct RayblasterArgs {
    /// File name to save the rendered scene to
    #[structopt(short, long, parse(from_os_str))]
    output_path: PathBuf,

    /// The name of the scene to render
    #[structopt(short, long)]
    scene_name: String,

    /// The width of the output image
    #[structopt(short, long)]
    width: usize,

    /// The height of the output image
    #[structopt(short, long)]
    height: usize,

    /// The FOV of the output image
    #[structopt(short, long)]
    fov: f64,

    /// The number of worker threads to spawn
    #[structopt(short, long)]
    threads: Option<usize>,

    /// The number of samples per pixel
    #[structopt(long, default_value = "1")]
    samples: u16,
}

fn main() -> Result<(), Error> {
    let args = RayblasterArgs::from_args();

    let mut buffer = vec![0; args.width * args.height * 3].into_boxed_slice();

    let scene = scene_lookup(&args.scene_name)
        .with_context(|| anyhow!("No primitives in {}", args.scene_name))?;

    if scene.primitives.is_empty() {
        return Err(anyhow!("Empty scene: {}", args.scene_name));
    }

    let samples = match args.samples {
        1 => SuperSamplingMode::SSAAX1,
        4 => SuperSamplingMode::SSAAX4,
        16 => SuperSamplingMode::SSAAX16,
        _ => return Err(anyhow!("Unsupported SSAA mode {}", args.samples)),
    };

    let t = Instant::now();
    let workers = args.threads.unwrap_or_else(num_cpus::get);

    let renderer = Renderer::new(args.width, args.height, workers, scene, args.fov, samples);
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
        args.output_path,
        &buffer,
        args.width as u32,
        args.height as u32,
        image::RGB(8),
    )
    .map_err(Error::from)
}
