use crate::materials::Material;
use cgmath::{InnerSpace, Vector3};

#[derive(Clone)]
pub struct SpecularMaterial {
    pub k_diff: f64,
    pub k_spec: f64,
    pub shininess: f64,
    pub diff_colour: Vector3<f64>,
    pub spec_colour: Vector3<f64>,
}

impl Material for SpecularMaterial {
    fn sample(
        &self,
        hit_normal: Vector3<f64>,
        ray_dir: Vector3<f64>,
        l: Vector3<f64>,
    ) -> Vector3<f64> {
        let half = (l + -ray_dir).normalize();

        let diffuse = if hit_normal.dot(l) > 0.0 {
            self.diff_colour * self.k_diff * hit_normal.dot(l)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        };

        let specular = self.spec_colour * self.k_spec * hit_normal.dot(half).powf(self.shininess);

        diffuse + specular
    }
}
