use cgmath::Vector3;
use scenes::Scene;
use primitives::{Primitive, Sphere};
use lights::Light;

pub fn get_scene() -> Scene {
    let mut prims: Vec<Box<Primitive>> = Vec::new();
    prims.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, 3.0), 3.5, Vector3::new(0, 0xFF, 0))));
    Scene {
        lights: Vec::new(),
        primitives: prims,
    }
}
