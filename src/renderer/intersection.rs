use crate::materials::Material;
use cgmath::Vector3;

pub struct Intersection<'a> {
    pub pos: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,
    pub material: &'a Box<dyn Material + Sync + 'a>,
}
