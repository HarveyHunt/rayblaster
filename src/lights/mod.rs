use cgmath::Vector3;

pub trait Light {
    fn pos(&self) -> Vector3<f64>;
    fn colour(&self) -> Vector3<f64>;
}
