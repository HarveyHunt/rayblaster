extern crate cgmath;
extern crate image;

mod renderer;
mod primitives;
mod lights;
mod scenes;

use scenes::{scene_lookup, Scene};
use renderer::render;
use cgmath::Vector3;

// TODO: Get this from user input
const WIDTH: usize = 640;
const HEIGHT: usize = 640;
const FOV: f64 = 30.0;

fn main() {
    let mut image: [Vector3<u8>; WIDTH * HEIGHT] = [Vector3::new(0, 0, 0); WIDTH * HEIGHT];
    let scene: Scene;
    // TODO: Get this from user input
    let scene_name = "sphere";

    match scene_lookup(scene_name) {
        Ok(s) => scene = s,
        Err(_) => panic!("Failed to load scene {}", scene_name),
    }

    render(&mut image, scene, WIDTH, HEIGHT, FOV);
}
