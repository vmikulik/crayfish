use std::f64::consts::PI;

use crayfish::camera::Camera;
use crayfish::canvas::Canvas;
use crayfish::colors::Color;
use crayfish::constants::EPSILON;
use crayfish::intersection::{intersect, hit, Intersectable};
use crayfish::materials::Scattered;
use crayfish::object::Object;
use crayfish::tuples::Tuple;
use crayfish::transformations::*;
use crayfish::ray::Ray;
use crayfish::groups::ObjectGroup;
use rand::Rng;

const ASPECT_RATIO: f64 = 16./9.;
const FOV: f64 = PI / 2.;

const IMAGE_HEIGHT: usize = 500;

const SAMPLES_PER_PIXEL: usize = 10;


fn ray_color(
    ray: &Ray,
    world: &impl Intersectable,
    min_t: f64,
) -> Color {

    if let Some(h) = hit(
        intersect(&ray, world).as_slice(), min_t,
    ) {
        return h.object.material
            .scatter(ray, h)
            .map(|Scattered{ attenuation, ray: scattered_ray }|
                attenuation * ray_color(&scattered_ray, world, 0.001)
            ).unwrap_or(Color::new(0., 0., 0.))
    };


    // Missed everything in the world: draw the sky.
    let t = (1. + ray.direction.unit().y) / 2.;
    let blue = Color::from_u8(135, 181, 235);
    let lightblue = Color::from_u8(135, 231, 235);
     return blue * t + lightblue * (1. - t);
}



fn main() {
    let mut rng = rand::thread_rng();

    // Canvas
    let image_width = (ASPECT_RATIO * IMAGE_HEIGHT as f64) as usize;
    let mut canvas = Canvas::new(image_width, IMAGE_HEIGHT);
    let pixel_height = 1. / IMAGE_HEIGHT as f64;
    let pixel_width = 1. / image_width as f64;

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
    world.add(Object::new_sphere().with_transform(
        scaling(100., 100., 100.)
        .translate(0., -101., 0.)
    ));

    // Main loop
    for y_pixel in 0..IMAGE_HEIGHT {
        if y_pixel % 50 == 0 {
            println!("Rendering row {} of {}", y_pixel, IMAGE_HEIGHT);
        }
        for x_pixel in 0..image_width {
            let y = y_pixel as f64 / IMAGE_HEIGHT as f64;
            let x = x_pixel as f64 / image_width as f64;

            let mut color = Color::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let x_sample = rng.gen_range(x..x+pixel_width);
                let y_sample = rng.gen_range(y..y+pixel_height);

                let ray = camera.cast_ray(x_sample, y_sample);
                color = color + ray_color(&ray, &world, 0.);
            }
            canvas.write_pixel(
                x_pixel,
                IMAGE_HEIGHT - 1 - y_pixel,  // Canvas uses an inverted y coordinate.
                (color * (1./SAMPLES_PER_PIXEL as f64)).gamma_encode()
            );
        }
    }

    std::fs::write("out.ppm", canvas.to_ppm())
        .expect("Unable to write file");
}
