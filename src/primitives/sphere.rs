use cgmath::Vector3;
use primitives::Primitive;
use renderer::Ray;

pub struct Sphere {
    pos: Vector3<f64>,
    radius: f64,
    // TODO: Replace a flat colour with a material
    colour: Vector3<u8>,
}

impl Sphere {
    pub fn new(pos: Vector3<f64>, radius: f64, colour: Vector3<u8>) -> Sphere {
        Sphere {
            pos: pos,
            radius: radius,
            colour: colour,
        }
    }
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> f64 {
        0.0
    }

    fn colour(&self) -> Vector3<u8> {
        self.colour
    }
}
