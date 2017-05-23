pub mod spherical_light;

use cgmath::Vector3;

pub use self::spherical_light::SphericalLight;

pub trait Light {
    fn center(&self) -> Vector3<f64>;
    fn colour(&self) -> Vector3<f64>;
}
