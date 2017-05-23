use materials::Material;
use cgmath::{Vector3, InnerSpace};

#[derive(Clone)]
pub struct DiffuseMaterial {
    // TODO: Add diffuse coefficient.
    pub colour: Vector3<f64>,
}

impl Material for DiffuseMaterial {
    fn sample(&self, hit_normal: Vector3<f64>, dir: Vector3<f64>, l: Vector3<f64>) -> Vector3<f64> {
        if hit_normal.dot(l) > 0.0 {
            self.colour * hit_normal.dot(l)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }
}
