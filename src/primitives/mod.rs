pub mod sphere;

use renderer::Ray;
use cgmath::Vector3;

pub use self::sphere::Sphere;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> f64;
    fn colour(&self) -> Vector3<u8>;
}
