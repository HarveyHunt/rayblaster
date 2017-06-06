use materials::Material;
use cgmath::{Vector3, InnerSpace};
use primitives::Primitive;
use renderer::{Ray, Intersection};
use std::f64::INFINITY;

pub struct Plane {
    pub center: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: Box<Material>,
}

impl Primitive for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let d = self.normal.dot(ray.direction);
        if d.abs() < 1e-6 {
            return None;
        }

        let t = (self.center - ray.origin).dot(self.normal) / d;
        if t < 0.0 {
            return None;
        }

        Some(Intersection {
            pos: ray.origin + ray.direction * t,
            normal: self.normal,
            distance: t,
            material: &self.material,
        })
    }
}
