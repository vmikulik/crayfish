use std::error::Error;
use std::time::Instant;

use crayfish::camera::Camera;
use crayfish::canvas::Canvas;
use crayfish::cli::{make_config, cli, Config};
use crayfish::colors::Color;
use crayfish::intersection::{intersect, hit, Intersectable};
use crayfish::materials::{Scattered, Metallic, Lambertian, Dielectric};
use crayfish::object::Object;
use crayfish::tuples::Tuple;
use crayfish::transformations::*;
use crayfish::ray::Ray;
use crayfish::groups::ObjectGroup;
use rand::Rng;


fn ray_color(
    ray: &Ray,
    world: &impl Intersectable,
    min_t: f64,
    depth: usize,
    config: &Config,
) -> Color {

    if depth > config.max_scatter_depth {
        return Color::new(0., 0., 0.)
    }

    if let Some(h) = hit(
        intersect(&ray, world).as_slice(), min_t,
    ) {
        return h.object.material
            .scatter(ray, h)
            .map(|Scattered{ attenuation, ray: scattered_ray }|
                attenuation * ray_color(&scattered_ray, world, 0.001, depth+1, config)
            ).unwrap_or(Color::new(0., 0., 0.))
    };


    // Missed everything in the world: draw the sky.
    let t = (1. + ray.direction.unit().y) / 2.;
    let blue = Color::from_u8(135, 181, 235);
    let lightblue = Color::from_u8(135, 231, 235);
     return blue * t + lightblue * (1. - t);
}



fn main() -> Result<(), Box<dyn Error>> {
    let config = make_config(cli().get_matches())?;
    let mut rng = rand::thread_rng();

    // Canvas
    let image_width = (config.aspect_ratio * config.image_height as f64) as usize;
    let mut canvas = Canvas::new(image_width, config.image_height);
    let pixel_height = 1. / config.image_height as f64;
    let pixel_width = 1. / image_width as f64;

    // Camera
    let camera = Camera::new(
        Tuple::point(0., 0., -3.),
        Tuple::point(0., 0., 0.),
        config.aspect_ratio,
        config.fov_radians,
    );

    // World
    let mut world = ObjectGroup::new();
    world.add(Object::new_sphere().with_transform(
        translation(0., 2., 0.,)
    ).with_material(
        Box::new(Lambertian::new(Color::from_u8(35, 21, 105)))
    ));
    world.add(Object::new_sphere().with_transform(
        translation(0., 0., 0.)
    ).with_material(
        Box::new(Dielectric::new(1.52))
    ));
    // .with_material(
    //     Box::new(Metallic::new(
    //         Color::new(0.3, 0.3, 0.3,), 0.1))
    // ));
    world.add(Object::new_sphere().with_transform(
        translation(2., 0., 0.)
    ).with_material(
        Box::new(Metallic::new(
            Color::new(1., 0.3, 0.3,), 0.02))
    ));
    world.add(Object::new_sphere().with_transform(
        scaling(100., 100., 100.)
        .translate(0., -101., 0.)
    ).with_material(
        Box::new(Lambertian::new(Color::from_u8(105, 63, 21)))
    ));

    // Main loop

    let start_time = Instant::now();
    let (from_row, to_row) = config.row_range;
    for y_pixel in from_row..to_row {
        if y_pixel % 1 == 0 {
            let duration = start_time.elapsed();
            println!(
                "Rendering row {} of {}, time elapsed: {:?}",
                y_pixel, to_row-from_row, duration
            );
        }
        for x_pixel in 0..canvas.width {
            let y = y_pixel as f64 / canvas.height as f64;
            let x = x_pixel as f64 / canvas.width as f64;

            let mut color = Color::new(0., 0., 0.);
            for _ in 0..config.rays_per_pixel {
                let x_sample = rng.gen_range(x..x+pixel_width);
                let y_sample = rng.gen_range(y..y+pixel_height);

                let ray = camera.cast_ray(x_sample, y_sample);
                let sample = ray_color(&ray, &world, 0., 0, &config);
                color = color + sample;
            }
            canvas.write_pixel(
                x_pixel,
                canvas.height - 1 - y_pixel,  // Canvas uses an inverted y coordinate.
                (color * (1./config.rays_per_pixel as f64)).gamma_encode()
            );
        }
    }

    let outpath = format!("{}.ppm", config.outfile);
    std::fs::write(outpath, canvas.to_ppm())
        .expect("Unable to write file");

    Ok(())
}
