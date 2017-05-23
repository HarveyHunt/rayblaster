pub mod diffuse;

use cgmath::Vector3;

pub use self::diffuse::DiffuseMaterial;

pub trait Material {
    fn sample(&self, hit_normal: Vector3<f64>, dir: Vector3<f64>, l: Vector3<f64>) -> Vector3<f64>;
}
