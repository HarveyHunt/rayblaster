use cgmath::Vector3;

#[derive(Clone, Copy)]
pub enum RayType {
    // TODO: Add more types so that we can count stats
    Primary,
    Shadow,
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    depth: u32,
    t: RayType,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, depth: u32, t: RayType) -> Self {
        Self {
            origin,
            direction,
            depth,
            t,
        }
    }

    pub fn from_origin(direction: Vector3<f64>, depth: u32, t: RayType) -> Self {
        Self {
            origin: Vector3::new(0.0, 0.0, 0.0),
            direction,
            depth,
            t,
        }
    }
}
