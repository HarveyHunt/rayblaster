pub mod spheres;

use anyhow::{anyhow, Error};

use crate::lights::Light;
use crate::primitives::Primitive;

pub struct Scene {
    pub lights: Vec<Box<dyn Light + Sync>>,
    pub primitives: Vec<Box<dyn Primitive + Sync>>,
}

pub fn scene_lookup(name: &str) -> Result<Scene, Error> {
    match name {
        "spheres" => Ok(spheres::get_scene()),
        _ => Err(anyhow!("Failed to load scene: {}", name)),
    }
}
