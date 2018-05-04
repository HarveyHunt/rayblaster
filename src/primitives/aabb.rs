use cgmath::{Vector3, InnerSpace};
use primitives::BoundingVolume;
use renderer::Ray;

pub struct AABB {
    pub min_bound: Vector3<f64>,
    pub max_bound: Vector3<f64>,
}

impl BoundingVolume for AABB {
    // Adapted from https://tavianator.com/fast-branchless-raybounding-box-intersections/
    fn intersect(&self, ray: &Ray) -> bool {
        let tx1 = (self.min_bound.x - ray.direction.x) * (1.0 / ray.direction.x);
        let tx2 = (self.max_bound.x - ray.direction.x) * (1.0 / ray.direction.x);

        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2);

        let ty1 = (self.min_bound.y - ray.direction.y) * (1.0 / ray.direction.y);
        let ty2 = (self.max_bound.y - ray.direction.y) * (1.0 / ray.direction.y);

        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));

        let tz1 = (self.min_bound.z - ray.direction.z) * (1.0 / ray.direction.z);
        let tz2 = (self.max_bound.z - ray.direction.z) * (1.0 / ray.direction.z);

        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));

        tmax >= tmin.max(0.0)
    }
}
