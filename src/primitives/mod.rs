pub mod sphere;
pub mod plane;

use renderer::{Ray, Intersection};
use cgmath::Vector3;

pub use self::sphere::Sphere;
pub use self::plane::Plane;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
