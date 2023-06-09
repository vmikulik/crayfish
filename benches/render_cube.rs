use std::f64::consts::PI;

use crayfish::{shapes::Shape, object::Object, cli::Config, raytrace::render_scene, camera::Camera, tuples::Tuple, groups::ObjectGroup, materials, transformations};
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn render_cube(c: &mut Criterion) {
    let cube = Object::new(Shape::Cube);

    let config = Config {
        aspect_ratio: 1.,
        fov_radians: PI / 2.,
        aperture_radius: 0.,
        outfile: "test".into(),
        image_height: 20,
        row_range: (0, 20),
        verbose: false,
        rays_per_pixel: 10,
        max_scatter_depth: 10,
    };

    let camera = Camera::new(
        Tuple::point(2., 2., 2.),
        Tuple::point(0., 0., 0.),
        config.aspect_ratio,
        config.fov_radians,
        None,
        config.aperture_radius,
    );

    c.bench_function("render_cube", |b| b.iter(|| {
        render_scene(black_box(&cube), &camera, &config);
    }));
}


fn render_cubes(c: &mut Criterion) {
    let mut world = ObjectGroup::new();
    world.add(
        Object::new(Shape::Cube)
        .with_material(Box::new(materials::Dielectric::new(1.52)))
    );
    world.add(
        Object::new(Shape::Cube)
        .with_material(Box::new(materials::Dielectric::new(1.52)))
        .with_transform(transformations::translation(-3., -3., -3.))
    );
    world.add(
        Object::new(Shape::Cube)
        .with_material(Box::new(materials::Dielectric::new(1.52)))
        .with_transform(transformations::translation(-6., -6., -6.))
    );

    let config = Config {
        aspect_ratio: 1.,
        fov_radians: PI / 2.,
        aperture_radius: 0.,
        outfile: "test".into(),
        image_height: 20,
        row_range: (0, 20),
        verbose: false,
        rays_per_pixel: 10,
        max_scatter_depth: 10,
    };

    let camera = Camera::new(
        Tuple::point(2., 2., 2.),
        Tuple::point(0., 0., 0.),
        config.aspect_ratio,
        config.fov_radians,
        None,
        config.aperture_radius,
    );

    c.bench_function("render_cubes", |b| b.iter(|| {
        render_scene(black_box(&world), &camera, &config);
    }));
}

criterion_group!(benches, render_cube, render_cubes);
criterion_main!(benches);