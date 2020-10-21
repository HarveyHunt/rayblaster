pub mod plane;
pub mod sphere;

use crate::renderer::{Intersection, Ray};

pub use self::plane::Plane;
pub use self::sphere::Sphere;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}
