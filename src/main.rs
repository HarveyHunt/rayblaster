extern crate cgmath;
extern crate image;

mod renderer;
mod primitives;
mod lights;
mod scenes;

use std::path::Path;
use scenes::{scene_lookup, Scene};
use renderer::render;
use cgmath::Vector3;
use image::{save_buffer, ColorType};

// TODO: Get this from user input
const WIDTH: usize = 640;
const HEIGHT: usize = 480;
const FOV: f64 = 80.0;

fn main() {
    let mut image: [Vector3<u8>; WIDTH * HEIGHT] = [Vector3::new(0, 0, 0); WIDTH * HEIGHT];
    let scene: Scene;
    // TODO: Get this from user input
    let scene_name = "sphere";
    let mut buffer = &mut [0; WIDTH * HEIGHT * 3];

    match scene_lookup(scene_name) {
        Ok(s) => scene = s,
        Err(_) => panic!("Failed to load scene {}", scene_name),
    }

    if scene.primitives.len() == 0 {
        panic!("No primitives in {}", scene_name);
    }

    render(&mut image, scene, WIDTH, HEIGHT, FOV);

    for (i, pixel) in image.iter().enumerate() {
        buffer[i * 3] = pixel.x;
        buffer[i * 3 + 1] = pixel.y;
        buffer[i * 3 + 2] = pixel.z;
    }

    // TODO: Get the filename from user input
    save_buffer(&Path::new("/tmp/test.png"),
                buffer,
                WIDTH as u32,
                HEIGHT as u32,
                image::RGB(8));
}
