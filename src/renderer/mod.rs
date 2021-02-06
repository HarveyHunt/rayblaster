pub mod intersection;
pub mod ray;

use std::cmp::{max, min, Ord};
use std::f64::INFINITY;
use std::iter::repeat;

use cgmath::{ElementWise, InnerSpace, Vector3};

use crate::primitives::Primitive;
use crate::scenes::Scene;

pub use self::intersection::Intersection;
pub use self::ray::Ray;

fn clamp<T: Ord>(val: T, minimum: T, maximum: T) -> T {
    max(minimum, min(val, maximum))
}

fn clamp_colour(r: f64, g: f64, b: f64) -> Vector3<u8> {
    Vector3::new(
        clamp((r * 255.0) as u32, 0, u8::max_value() as u32) as u8,
        clamp((g * 255.0) as u32, 0, u8::max_value() as u32) as u8,
        clamp((b * 255.0) as u32, 0, u8::max_value() as u32) as u8,
    )
}

pub struct Renderer {
    width: usize,
    height: usize,
    workers: usize,
    scene: Scene,
    samples: SuperSamplingMode,
    aspect_ratio: f64,
    scale: f64,
}

#[derive(Debug, Copy, Clone)]
pub enum SuperSamplingMode {
    SSAAX1 = 1,
    SSAAX4 = 4,
    SSAAX16 = 16,
}

impl SuperSamplingMode {
    pub fn sample_coords(&self) -> std::slice::Iter<'_, f64> {
        match self {
            SuperSamplingMode::SSAAX1 => [0.5].iter(),
            SuperSamplingMode::SSAAX4 => [0.25, 0.75].iter(),
            SuperSamplingMode::SSAAX16 => [0.125, 0.375, 0.625, 0.875].iter(),
        }
    }
}

impl Renderer {
    pub fn new(
        width: usize,
        height: usize,
        workers: usize,
        scene: Scene,
        fov: f64,
        samples: SuperSamplingMode,
    ) -> Self {
        Self {
            width,
            height,
            workers,
            scene,
            samples,
            aspect_ratio: (width / height) as f64,
            scale: (fov.to_radians() * 0.5).tan(),
        }
    }

    pub fn render(&self) -> Vec<Vector3<u8>> {
        // Split in width factored chunks in order to remain cache friendly.
        // FIXME: This will break with weird sizes that cause a fractional
        // chunk_size.
        let mut frame: Vec<Vector3<u8>> = repeat(Vector3::new(0, 0, 0))
            .take(self.width * self.height)
            .collect();

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
        let mut sample_index;
        let mut sample_buf: Vec<Vector3<f64>> =
            vec![Vector3::new(0.0, 0.0, 0.0); self.samples as usize];

        for y in 0..chunk_height {
            for x in 0..self.width {
                sample_index = 0;
                for x_sample in self.samples.sample_coords() {
                    for y_sample in self.samples.sample_coords() {
                        let cx = (2.0 * (((x as f64 + x_sample) as f64) * inv_width) - 1.0)
                            * self.aspect_ratio
                            * self.scale;

                        let cy = (1.0 - 2.0 * (((y + y_offset) as f64 + y_sample) * inv_height))
                            * self.scale;

                        let dir = Vector3::new(cx, cy, -1.0).normalize();
                        let ray = Ray::from_origin(dir);

                        sample_buf[sample_index] = self.trace(ray);
                        sample_index += 1;
                    }
                }

                let sum_col = sample_buf
                    .iter()
                    .fold(Vector3::new(0.0, 0.0, 0.0), |sum, val| sum + val)
                    / (self.samples as u32) as f64;

                chunk[pixel] = clamp_colour(sum_col.x, sum_col.y, sum_col.z);
                pixel += 1;
            }
        }
    }

    pub fn trace(&self, ray: Ray) -> Vector3<f64> {
        let colour = Vector3::new(0.0, 0.0, 0.0);
        let int: Intersection;

        match self.trace_primary(ray) {
            Some((i, _p)) => {
                int = i;
            }
            None => return colour,
        };

        let col: Vector3<f64> =
            self.scene
                .lights
                .iter()
                .fold(Vector3::new(0.0, 0.0, 0.0), |acc, light| {
                    let l = (light.center() - int.pos).normalize();
                    // Add a bias to prevent shadow acne.
                    // TODO: Experiment to find a good value.
                    let bias = Vector3::new(1e-6, 1e-6, 1e-6);
                    let shadow_ray = Ray::new(int.pos + bias, l);

                    if !self.trace_shadow(shadow_ray) {
                        acc + int
                            .material
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
            Some((_int, _prim)) => true,
            None => false,
        }
    }

    // TODO: All of this boxiness feels gross...
    fn trace_primary(&self, ray: Ray) -> Option<(Intersection, &Box<dyn Primitive + Sync>)> {
        let mut tnear: f64 = INFINITY;
        let mut result: Option<(Intersection, &Box<dyn Primitive + Sync>)> = None;

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
