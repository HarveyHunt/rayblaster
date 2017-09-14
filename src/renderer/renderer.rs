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
}

impl Renderer {
    pub fn new(width: usize, height: usize, workers: usize, scene: Scene, fov: f64) -> Renderer {
        Renderer {
            width: width,
            height: height,
            workers: workers,
            scene: scene,
            fov: fov,
        }
    }

    pub fn render(&self) -> Vec<Vector3<u8>> {
        // Split in width factored chunks in order to remain cache friendly.
        // FIXME: This will break with weird sizes that cause a fractional
        // chunk_size.
        //
        // We should be ensuring that an x offset never needs to be applied due to
        // remaining pixels to be processed
        let mut frame: Vec<Vector3<u8>> =
            repeat(Vector3::new(0, 0, 0)).take(self.width * self.height).collect();

        let chunk_size = (self.height / self.workers) * self.width;
        let mut i = 0;

        println!("Rendering using {} workers", self.workers);

        crossbeam::scope(|scope| {
            for chunk in frame.chunks_mut(chunk_size) {
                scope.spawn(move || {
                    self.render_chunk(chunk, self.width, self.height / self.workers, 0, i)
                });
                i += self.height / self.workers;
            }
        });

        frame
    }

    pub fn render_chunk(&self,
                        chunk: &mut [Vector3<u8>],
                        chunk_width: usize,
                        chunk_height: usize,
                        x_offset: usize,
                        y_offset: usize) {
        let mut pixel = y_offset * x_offset;
        let aspect_ratio = (self.width / self.height) as f64;

        let scale = (self.fov.to_radians() * 0.5).tan();
        // Multiply is cheaper than divide, so use the inverses in the main loop.
        let inv_width = 1.0 / self.width as f64;
        let inv_height = 1.0 / self.height as f64;
        let origin = Vector3::new(0.0, 0.0, 0.0);

        for y in 0..chunk_height {
            for x in 0..chunk_width {
                let cx = (2.0 * (((x + x_offset) as f64 + 0.5) * inv_width) - 1.0) * aspect_ratio *
                         scale;
                let cy = (1.0 - 2.0 * (((y + y_offset) as f64 + 0.5) * inv_height)) * scale;

                let dir = Vector3::new(cx, cy, -1.0).normalize();
                let ray = Ray::new(origin, dir, MAX_DEPTH, RayType::Primary);

                let colour = self.trace(ray);

                chunk[pixel] = clamp_colour(colour);
                pixel += 1;
            }
        }
    }

    pub fn trace(&self, ray: Ray) -> Vector3<f64> {
        let mut colour = Vector3::new(0.0, 0.0, 0.0);
        let prim: &Box<Primitive + Sync>;
        let int: Intersection;

        match self.trace_primary(ray) {
            Some((i, p)) => {
                int = i;
                prim = p
            }
            None => return colour,
        };

        let mut col: Vector3<f64> =
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

        for prim in self.scene.primitives.iter() {
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
}
