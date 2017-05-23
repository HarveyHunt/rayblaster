pub mod sphere;

use renderer::{Ray, Intersection};
use cgmath::Vector3;

pub use self::sphere::Sphere;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
