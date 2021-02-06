use cgmath::{Vector3, Zero};

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn from_origin(direction: Vector3<f64>) -> Self {
        Self {
            origin: Vector3::zero(),
            direction,
        }
    }
}
