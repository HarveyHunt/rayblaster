use cgmath::{Vector3, InnerSpace};
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
        let r_squared = self.radius.powi(2);
        let l = self.pos - ray.origin;
        let tca = l.dot(ray.direction);
        if tca < 0.0 {
            return -1.0;
        }

        let d2 = l.dot(l) - (tca * tca);
        if d2 < r_squared {
            return -1.0;
        }

        let thc = (r_squared - d2).sqrt();

        let t0 = tca - thc;
        let t1 = tca + thc;

        // FIXME
        0.0
    }

    fn colour(&self) -> Vector3<u8> {
        self.colour
    }
}
