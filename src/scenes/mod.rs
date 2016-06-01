pub mod sphere;

use primitives::Primitive;
use lights::Light;

pub struct Scene {
    pub lights: Vec<Box<Light>>,
    pub primitives: Vec<Box<Primitive>>,
}

pub fn scene_lookup(name: &str) -> Result<Scene, &'static str> {
    match name {
        "sphere" => Ok((sphere::get_scene())),
        _ => Err("Failed to load scene"),
    }
}
