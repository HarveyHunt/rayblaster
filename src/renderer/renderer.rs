use scenes::Scene;
use cgmath::{Vector3, InnerSpace};
use renderer::{Ray, RayType};

const MAX_DEPTH: u32 = 5;

pub fn render(buffer: &mut [Vector3<u8>], scene: Scene, width: usize, height: usize, fov: f64) {
    let mut pixel = 0;
    let aspect_ratio = (width / height) as f64;
    let scale = (fov.to_radians() * 0.5).tan();
    // Multiply is cheaper than divide, so use the inverses in the main loop.
    let inv_width = 1.0 / width as f64;
    let inv_height = 1.0 / height as f64;
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for y in 0..height {
        for x in 0..width {
            let cx = (2.0 * ((x as f64 + 0.5) * inv_width) - 1.0) * aspect_ratio * scale;
            let cy = (1.0 - 2.0 * ((y as f64 * 0.5) * inv_height)) * scale;

            let dir = Vector3::new(cx, cy, -1.0).normalize();
            let ray = Ray::new(origin, dir, MAX_DEPTH, RayType::Primary);

            let colour = trace(ray, &scene);
            buffer[pixel] = colour;
            pixel += 1;
        }
    }
}

pub fn trace(ray: Ray, scene: &Scene) -> Vector3<u8> {
    let mut t: f64;
    let mut colour = Vector3::new(0, 0, 0);

    for prim in scene.primitives.iter() {
        t = prim.intersect(&ray);
        if t > 0.0 {
            colour = prim.colour();
        }
    }

    colour
}
