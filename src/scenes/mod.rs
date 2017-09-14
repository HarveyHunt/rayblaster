pub mod spheres;

use primitives::Primitive;
use lights::Light;

pub struct Scene {
    pub lights: Vec<Box<Light + Sync>>,
    pub primitives: Vec<Box<Primitive + Sync>>,
}

pub fn scene_lookup(name: &str) -> Result<Scene, &'static str> {
    match name {
        "spheres" => Ok((spheres::get_scene())),
        _ => Err("Failed to load scene"),
    }
}
