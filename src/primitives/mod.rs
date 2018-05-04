pub mod sphere;
pub mod plane;
pub mod aabb;

use cgmath::Vector3;
use renderer::{Ray, Intersection};

pub use self::sphere::Sphere;
pub use self::plane::Plane;
pub use self::aabb::AABB;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn fast_intersect(&self, ray: &Ray) -> bool;
}

pub trait BoundingVolume {
    fn intersect(&self, ray: &Ray) -> bool;
}
