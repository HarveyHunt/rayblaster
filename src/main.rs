extern crate cgmath;
extern crate image;

mod renderer;
mod primitives;
mod lights;
mod scenes;

use scenes::{scene_lookup, Scene};
use renderer::render;

// TODO: Get this from user input
const WIDTH: usize = 640;
const HEIGHT: usize = 640;

fn main() {
    let mut image: [u8; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    let scene: Scene;
    // TODO: Get this from user input
    let scene_name = "sphere";

    match scene_lookup(scene_name) {
        Ok(s) => scene = s,
        Err(_) => panic!("Failed to load scene {}", scene_name),
    }

    render(&mut image, scene, WIDTH, HEIGHT);
}
