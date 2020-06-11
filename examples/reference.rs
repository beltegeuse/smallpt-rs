extern crate smallpt;
extern crate minifb;

use smallpt::*;
use minifb::{Key, Window, WindowOptions};

fn main() {
    const NUM_SAMPLES: u32 = 16;
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;

    let mut backbuffer = vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];

    let mut scene = Scene::init();

    // Spheres
    // Mirror
    scene.add(Box::new(Sphere::new(
        16.5,
        Vec3::new(27.0, 16.5, 47.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0), BSDF::Mirror),
    )));

    // Glass
    scene.add(Box::new(Sphere::new(
        16.5,
        Vec3::new(73.0, 16.5, 78.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0), BSDF::Glass),
    )));

    // Planes
    // Bottom
    scene.add(Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
    )));

    // Left
    scene.add(Box::new(Plane::new(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.75, 0.25, 0.25), BSDF::Diffuse),
    )));

    // Right
    scene.add(Box::new(Plane::new(
        Vec3::new(99.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.25, 0.25, 0.75), BSDF::Diffuse),
    )));

    // Front
    scene.add(Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
    )));

    // Back
    scene.add(Box::new(Plane::new(
        Vec3::new(0.0, 0.0, 170.0),
        Vec3::new(0.0, 0.0, -1.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), BSDF::Diffuse),
    )));

    // Top
    scene.add(Box::new(Plane::new(
        Vec3::new(0.0, 81.6, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Material::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.75, 0.75, 0.75), BSDF::Diffuse),
    )));

    // Light (emissive rectangle)
    scene.add(Box::new(Rectangle::new(
        Vec3::new(50.0, 81.5, 50.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        33.0,
        33.0,
        Material::new(Vec3::new(12.0, 12.0, 12.0), Vec3::new(0.0, 0.0, 0.0), BSDF::Diffuse),
    )));

    let camera = Camera {
        origin: Vec3::new(50.0, 50.0, 200.0),
        forward: Vec3::new(0.0, -0.05, -1.0).normalize(),
        right: Vec3::new(1.0, 0.0, 0.0).normalize(),
        up: Vec3::new(0.0, 1.0, 0.0).normalize(),
    };

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("smallpt in Rust", WIDTH, HEIGHT, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Render
    let mut iterations = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut current_backbuffer = vec![Vec3::new(0.0, 0.0, 0.0); WIDTH * HEIGHT];
        let mut num_rays = 0;
        trace(&scene, &camera, WIDTH, HEIGHT, NUM_SAMPLES, &mut current_backbuffer, &mut num_rays);
        for (pdest, p) in backbuffer.iter_mut().zip(current_backbuffer.iter()) {
            *pdest = (*pdest * iterations as f32 + *p) / (iterations + 1) as f32;
        }
        iterations += 1;

        for i in 0..WIDTH * HEIGHT {
            let color = saturate(tonemap(backbuffer[i]));

            let r = (color.x * 255.0).round() as u32;
            let g = (color.y * 255.0).round() as u32;
            let b = (color.z * 255.0).round() as u32;

            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
