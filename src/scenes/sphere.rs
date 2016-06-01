use scenes::Scene;
use primitives::Primitive;
use lights::Light;

pub fn get_scene() -> Scene {
    Scene {
        lights: Vec::new(),
        primitives: Vec::new(),
    }
}
