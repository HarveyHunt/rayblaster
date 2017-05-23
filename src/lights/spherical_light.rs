use cgmath::Vector3;
use lights::Light;

pub struct SphericalLight {
    pub center: Vector3<f64>,
    pub colour: Vector3<f64>,
}

impl Light for SphericalLight {
    fn center(&self) -> Vector3<f64> {
        self.center
    }

    fn colour(&self) -> Vector3<f64> {
        self.colour
    }
}
