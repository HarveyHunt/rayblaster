use scenes::Scene;
use crossbeam;
use cgmath::{Vector3, InnerSpace, ElementWise};
use renderer::{Ray, RayType, Intersection};
use primitives::Primitive;
use std::f64::INFINITY;
use std::iter::repeat;
use std::cmp::{max, min, Ord};

const MAX_DEPTH: u32 = 5;

fn clamp<T: Ord>(val: T, minimum: T, maximum: T) -> T {
    max(minimum, min(val, maximum))
}

fn clamp_colour(colour: Vector3<f64>) -> Vector3<u8> {
    Vector3::new(clamp((colour.x * 255.0) as u32, 0, u8::max_value() as u32) as u8,
                 clamp((colour.y * 255.0) as u32, 0, u8::max_value() as u32) as u8,
                 clamp((colour.z * 255.0) as u32, 0, u8::max_value() as u32) as u8)
}

pub struct Renderer {
    width: usize,
    height: usize,
    workers: usize,
    scene: Scene,
    fov: f64,
    aspect_ratio: f64,
    scale: f64,
}

impl Renderer {
    pub fn new(width: usize, height: usize, workers: usize, scene: Scene, fov: f64) -> Renderer {
        Renderer {
            width: width,
            height: height,
            workers: workers,
            scene: scene,
            fov: fov,
            aspect_ratio: (width / height) as f64,
            scale: (fov.to_radians() * 0.5).tan(),
        }
    }

    pub fn render(&self) -> Vec<Vector3<u8>> {
        // Split in width factored chunks in order to remain cache friendly.
        // FIXME: This will break with weird sizes that cause a fractional
        // chunk_size.
        let mut frame: Vec<Vector3<u8>> =
            repeat(Vector3::new(0, 0, 0)).take(self.width * self.height).collect();

        println!("Rendering using {} workers", self.workers);

        let chunk_lines = self.height / self.workers;
        let chunk_size = chunk_lines * self.width;
        let mut start_line = 0;

        crossbeam::scope(|scope| {
            for chunk in frame.chunks_mut(chunk_size) {
                if self.height - start_line < chunk_lines {
                    scope.spawn(move || {
                        self.render_chunk(chunk, self.height - start_line, start_line)
                    });
                } else {
                    scope.spawn(move || self.render_chunk(chunk, chunk_lines, start_line));
                }
                start_line += chunk_lines;
            }
        });

        frame
    }

    pub fn render_chunk(&self, chunk: &mut [Vector3<u8>], chunk_height: usize, y_offset: usize) {
        // Multiply is cheaper than divide, so use the inverses in the main loop.
        let inv_width = 1.0 / self.width as f64;
        let inv_height = 1.0 / self.height as f64;
        let mut pixel = 0;

        for y in 0..chunk_height {
            for x in 0..self.width {
                let cx = (2.0 * ((x as f64 + 0.5) * inv_width) - 1.0) * self.aspect_ratio *
                         self.scale;
                let cy = (1.0 - 2.0 * (((y + y_offset) as f64 + 0.5) * inv_height)) * self.scale;

                let dir = Vector3::new(cx, cy, -1.0).normalize();
                let ray = Ray::from_origin(dir, MAX_DEPTH, RayType::Primary);

                let colour = self.trace(ray);

                chunk[pixel] = clamp_colour(colour);
                pixel += 1;
            }
        }
    }

    pub fn trace(&self, ray: Ray) -> Vector3<f64> {
        let colour = Vector3::new(0.0, 0.0, 0.0);
        let prim: &Box<Primitive + Sync>;
        let int: Intersection;

        match self.trace_primary(ray) {
            Some((i, p)) => {
                int = i;
                prim = p
            }
            None => return colour,
        };

        let col: Vector3<f64> =
            self.scene.lights.iter().fold(Vector3::new(0.0, 0.0, 0.0), |acc, light| {
                let l = (light.center() - int.pos).normalize();
                // Add a bias to prevent shadow acne.
                // TODO: Experiment to find a good value.
                let bias = Vector3::new(1e-6, 1e-6, 1e-6);
                let shadow_ray = Ray::new(int.pos + bias, l, 1, RayType::Shadow);

                if !self.trace_shadow(shadow_ray) {
                    acc +
                    int.material
                        .sample(int.normal.normalize(), ray.direction, l)
                        .mul_element_wise(light.colour())
                } else {
                    acc
                }
            });

        col
    }

    // TODO: Once translucent objects are implemented, hitting one will modify the colour of
    // primitive we are checking shadows for.
    fn trace_shadow(&self, ray: Ray) -> bool {
        match self.trace_primary(ray) {
            Some((int, prim)) => true,
            None => false,
        }
    }

    // TODO: All of this boxiness feels gross...
    fn trace_primary(&self, ray: Ray) -> Option<(Intersection, &Box<Primitive + Sync>)> {
        let mut tnear: f64 = INFINITY;
        let mut result: Option<(Intersection, &Box<Primitive + Sync>)> = None;

        for prim in &self.scene.primitives {
            if let Some(int) = prim.intersect(&ray) {
                if int.distance < tnear {
                    tnear = int.distance;
                    result = Some((int, prim))
                }
            };
        }

        result
    }
}
