use scenes::Scene;
use cgmath::{Vector3, InnerSpace};
use renderer::{Ray, RayType, Intersection};
use primitives::Primitive;
use std::f64::INFINITY;

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

            buffer[pixel] = Vector3::new((colour.x * 255.0) as u8,
                                         (colour.y * 255.0) as u8,
                                         (colour.z * 255.0) as u8);
            pixel += 1;
        }
    }
}

pub fn trace(ray: Ray, scene: &Scene) -> Vector3<f64> {
    let mut colour = Vector3::new(0.0, 0.0, 0.0);
    let prim: &Box<Primitive>;
    let int: Intersection;

    match trace_primary(ray, scene) {
        Some((i, p)) => {
            int = i;
            prim = p
        }
        None => return colour,
    };

    let mut col: Vector3<f64> =
        scene.lights.iter().fold(Vector3::new(0.0, 0.0, 0.0), |acc, light| {
            let l = (light.center() - int.pos).normalize();
            // TODO: Mix the light's colour in.
            acc + int.material.sample(int.normal.normalize(), ray.direction, l)
        });

    col
}

// TODO: All of this boxiness feels gross...
fn trace_primary(ray: Ray, scene: &Scene) -> Option<(Intersection, &Box<Primitive>)> {
    let mut tnear: f64 = INFINITY;
    let mut result: Option<(Intersection, &Box<Primitive>)> = None;

    for prim in scene.primitives.iter() {
        match prim.intersect(&ray) {
            Some(int) => {
                if int.distance < tnear {
                    tnear = int.distance;
                    result = Some((int, prim))
                }
            }
            None => {}
        };
    }

    result
}
