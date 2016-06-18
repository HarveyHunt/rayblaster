use cgmath::Vector3;

pub struct Intersection {
    pub pos: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,
}
