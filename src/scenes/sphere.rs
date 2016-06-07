use cgmath::Vector3;
use scenes::Scene;
use primitives::{Primitive, Sphere};
use lights::Light;

pub fn get_scene() -> Scene {
    let mut prims: Vec<Box<Primitive>> = Vec::new();
    prims.push(Box::new(Sphere::new(Vector3::new(30.0, 10.0, -50.0),
                                    3.5,
                                    Vector3::new(0xFF, 0x00, 0))));

    prims.push(Box::new(Sphere::new(Vector3::new(-20.0, 10.0, -50.0),
                                    7.0,
                                    Vector3::new(0x00, 0xFF, 0x00))));

    prims.push(Box::new(Sphere::new(Vector3::new(-30.0, 70.0, -150.0),
                                    15.0,
                                    Vector3::new(0x00, 0x00, 0xFF))));
    Scene {
        lights: Vec::new(),
        primitives: prims,
    }
}
