use std::time::Instant;
use rand::Rng;

use crate::{
    ray::Ray,
    intersection::{
        Intersectable, hit, intersect
    },
    cli::Config,
    colors::Color,
    materials::Scattered,
    canvas::Canvas,
    camera::Camera,
};


pub fn ray_color(
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
        intersect(ray, world).as_slice(), min_t,
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
     blue * t + lightblue * (1. - t)
}


pub fn render_scene(
    world: impl Intersectable,
    camera: &Camera,
    config: &Config,
) -> Canvas {
    let mut rng = rand::thread_rng();

    let image_width = (config.aspect_ratio * config.image_height as f64) as usize;
    let mut canvas = Canvas::new(image_width, config.image_height);
    let pixel_height = 1. / config.image_height as f64;
    let pixel_width = 1. / image_width as f64;

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
                let sample = ray_color(&ray, &world, 0., 0, config);
                color = color + sample;
            }
            canvas.write_pixel(
                x_pixel,
                canvas.height - 1 - y_pixel,  // Canvas uses an inverted y coordinate.
                (color * (1./config.rays_per_pixel as f64)).gamma_encode()
            );
        }
    }

    canvas
}