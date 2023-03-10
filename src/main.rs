use std::f64::consts::PI;

use crayfish::camera::Camera;
use crayfish::canvas::Canvas;
use crayfish::colors::Color;
use crayfish::intersection::{intersect, hit, Intersectable};
use crayfish::object::Object;
use crayfish::tuples::Tuple;
use crayfish::transformations::*;
use crayfish::ray::Ray;
use crayfish::groups::ObjectGroup;

const ASPECT_RATIO: f64 = 16./9.;
const FOV: f64 = PI / 3.;

const IMAGE_HEIGHT: usize = 500;


fn ray_color(ray: &Ray, world: &impl Intersectable) -> Color {

    if let Some(h) = hit(
        intersect(&ray, world).as_slice(), 0.
    ) {
        let n = h.object.normal_at(ray.position(h.t));
        return (Color::new(n.x + 1., n.y + 1., n.z + 1.)) * 0.5
    };


    // Missed everything in the world: draw the sky.
    let t = (1. + ray.direction.unit().y) / 2.;
    let blue = Color::from_u8(135, 181, 235);
    let lightblue = Color::from_u8(135, 231, 235);
     return blue * t + lightblue * (1. - t);
}



fn main() {

    // Canvas
    let image_width = (ASPECT_RATIO * IMAGE_HEIGHT as f64) as usize;
    let mut canvas = Canvas::new(image_width, IMAGE_HEIGHT);

    // Camera
    let camera = Camera::new(
        Tuple::point(0., 0., -3.),
        Tuple::point(0., 0., 0.),
        ASPECT_RATIO,
        FOV,
    );

    // World
    let mut world = ObjectGroup::new();
    world.add(Object::new_sphere().with_transform(
        translation(0., 0., 0.)
    ));
    world.add(Object::new_sphere().with_transform(
        translation(0., 2., 0.,)
    ));

    // Main loop
    for y_pixel in 0..IMAGE_HEIGHT {
        if y_pixel % 50 == 0 {
            println!("Rendering row {} of {}", y_pixel, IMAGE_HEIGHT);
        }
        for x_pixel in 0..image_width {
            let y = y_pixel as f64 / IMAGE_HEIGHT as f64;
            let x = x_pixel as f64 / image_width as f64;

            let ray = camera.cast_ray(x, y);
            let color = ray_color(&ray, &world);
            canvas.write_pixel(x_pixel, y_pixel, color);
        }
    }

    std::fs::write("out.ppm", canvas.to_ppm())
        .expect("Unable to write file");
}
