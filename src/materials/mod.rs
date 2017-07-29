pub mod diffuse;
pub mod specular;

use cgmath::Vector3;

pub use self::diffuse::DiffuseMaterial;
pub use self::specular::SpecularMaterial;

pub trait Material {
    fn sample(&self, hit_normal: Vector3<f64>, ray_dir: Vector3<f64>, l: Vector3<f64>) -> Vector3<f64>;
}
