pub mod spheres;

use crate::lights::Light;
use crate::primitives::Primitive;

pub struct Scene {
    pub lights: Vec<Box<dyn Light + Sync>>,
    pub primitives: Vec<Box<dyn Primitive + Sync>>,
}

pub fn scene_lookup(name: &str) -> Result<Scene, &'static str> {
    match name {
        "spheres" => Ok(spheres::get_scene()),
        _ => Err("Failed to load scene"),
    }
}
