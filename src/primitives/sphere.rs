use crate::materials::Material;
use crate::primitives::Primitive;
use crate::renderer::{Intersection, Ray};
use cgmath::{InnerSpace, Vector3};

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Box<dyn Material + Sync>,
}

impl Primitive for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let r_squared = self.radius.powi(2);
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction);
        if tca < 0.0 {
            return None;
        }

        let d2 = l.dot(l) - (tca * tca);
        if d2 > r_squared {
            return None;
        }

        let thc = (r_squared - d2).sqrt();

        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 {
            t0 = t1;
        }

        let pos = ray.origin + ray.direction * t0;
        let normal = pos - self.center;

        Some(Intersection {
            pos,
            normal,
            distance: t0,
            material: &self.material,
        })
    }
}
