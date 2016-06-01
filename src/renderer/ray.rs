use cgmath::Vector3;

pub enum RayType {
    // TODO: Add more types so that we can count stats
    Primary,
}

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    depth: u32,
    t: RayType,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, depth: u32, t: RayType) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            depth: depth,
            t: t,
        }
    }
}
