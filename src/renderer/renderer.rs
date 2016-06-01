use scenes::Scene;
use cgmath::Vector3;
use renderer::Ray;

pub fn render(buffer: &mut [u8], scene: Scene, width: usize, height: usize) {
    for x in 0..width {
        for y in 0..height {
        }
    }
}

pub fn trace(ray: Ray, scene: &Scene) -> Vector3<f64> {
    Vector3::new(0.0, 0.0, 0.0)
}
