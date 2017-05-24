use cgmath::Vector3;
use scenes::Scene;
use materials::DiffuseMaterial;
use primitives::{Primitive, Sphere};
use lights::{Light, SphericalLight};

pub fn get_scene() -> Scene {
    let mut prims: Vec<Box<Primitive>> = Vec::new();

    prims.push(Box::new(Sphere {
        center: Vector3::new(30.0, 10.0, -50.0),
        radius: 3.5,
        material: Box::new(DiffuseMaterial { colour: Vector3::new(1.0, 0.0, 0.0) }),
    }));

    prims.push(Box::new(Sphere {
        center: Vector3::new(-15.0, 30.0, -50.0),
        radius: 7.0,
        material: Box::new(DiffuseMaterial { colour: Vector3::new(0.0, 1.0, 0.0) }),
    }));

    prims.push(Box::new(Sphere {
        center: Vector3::new(-30.0, 70.0, -150.0),
        radius: 15.0,
        material: Box::new(DiffuseMaterial { colour: Vector3::new(0.0, 0.0, 1.0) }),
    }));

    prims.push(Box::new(Sphere {
        center: Vector3::new(0.0, 50.0, -150.0),
        radius: 25.0,
        material: Box::new(DiffuseMaterial { colour: Vector3::new(1.0, 1.0, 1.0) }),
    }));

    let mut ls: Vec<Box<Light>> = Vec::new();
    ls.push(Box::new(SphericalLight {
        center: Vector3::new(-25.0, 10.0, -10.0),
        colour: Vector3::new(1.0, 1.0, 1.0),
    }));


    Scene {
        lights: ls,
        primitives: prims,
    }
}
