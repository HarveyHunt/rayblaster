use cgmath::Vector3;
use scenes::Scene;
use materials::SpecularMaterial;
use primitives::{Primitive, Sphere, Plane};
use lights::{Light, SphericalLight};

pub fn get_scene() -> Scene {
    let mut prims: Vec<Box<Primitive>> = Vec::new();

    prims.push(Box::new(Plane {
        center: Vector3::new(0.0, -5.0, 0.0),
        normal: Vector3::new(0.0, 1.0, 0.0),
        material: Box::new(SpecularMaterial {
            k_diff: 0.0,
            k_spec: 1.0,
            shininess: 20.0,
            diff_colour: Vector3::new(1.0, 1.0, 1.0),
            spec_colour: Vector3::new(1.0, 1.0, 1.0),
        }),
    }));

    prims.push(Box::new(Sphere {
        center: Vector3::new(-10.0, 10.0, -25.0),
        radius: 2.0,
        material: Box::new(SpecularMaterial {
            k_diff: 0.6,
            k_spec: 0.4,
            shininess: 10.0,
            diff_colour: Vector3::new(1.0, 0.0, 1.0),
            spec_colour: Vector3::new(1.0, 1.0, 1.0),
        }),
    }));

    prims.push(Box::new(Sphere {
        center: Vector3::new(0.0, 0.0, -10.0),
        radius: 2.0,
        material: Box::new(SpecularMaterial {
            k_diff: 0.3,
            k_spec: 0.7,
            shininess: 10.0,
            diff_colour: Vector3::new(0.0, 0.0, 1.0),
            spec_colour: Vector3::new(1.0, 1.0, 1.0),
        }),
    }));

    prims.push(Box::new(Sphere {
        center: Vector3::new(7.0, 8.0, -15.0),
        radius: 3.0,
        material: Box::new(SpecularMaterial {
            k_diff: 0.7,
            k_spec: 0.4,
            shininess: 10.0,
            diff_colour: Vector3::new(0.0, 0.0, 1.0),
            spec_colour: Vector3::new(1.0, 1.0, 1.0),
        }),
    }));

    prims.push(Box::new(Sphere {
        center: Vector3::new(-12.0, 7.0, -15.0),
        radius: 1.5,
        material: Box::new(SpecularMaterial {
            k_diff: 0.3,
            k_spec: 0.7,
            shininess: 10.0,
            diff_colour: Vector3::new(1.0, 0.0, 0.0),
            spec_colour: Vector3::new(1.0, 1.0, 1.0),
        }),
    }));

    let mut ls: Vec<Box<Light>> = Vec::new();
    ls.push(Box::new(SphericalLight {
        center: Vector3::new(25.0, 20.0, 10.0),
        colour: Vector3::new(1.0, 1.0, 1.0),
    }));

    ls.push(Box::new(SphericalLight {
        center: Vector3::new(-20.0, 10.0, -50.0),
        colour: Vector3::new(0.0, 0.0, 1.0),
    }));

    ls.push(Box::new(SphericalLight {
        center: Vector3::new(20.0, 1.0, -40.0),
        colour: Vector3::new(1.0, 0.0, 0.0),
    }));

    Scene {
        lights: ls,
        primitives: prims,
    }
}
