use std::error::Error;

use crayfish::camera::Camera;
use crayfish::cli::{make_config, cli};
use crayfish::colors::Color;
use crayfish::materials::{Metallic, Lambertian, Dielectric};
use crayfish::object::Object;
use crayfish::tuples::Tuple;
use crayfish::transformations::*;
use crayfish::groups::ObjectGroup;
use crayfish::shapes::Shape;
use crayfish::raytrace::render_scene;



fn main() -> Result<(), Box<dyn Error>> {
    let config = make_config(cli().get_matches())?;

    // Camera
    let camera = Camera::new(
        Tuple::point(4., 4., -10.),
        Tuple::point(1., 1., 0.),
        config.aspect_ratio,
        config.fov_radians,
        None,
        config.aperture_radius,
    );

    // World
    let mut world = ObjectGroup::new();
    world.add(Object::new_sphere().with_transform(
        translation(0., 2., 0.,)
    ).with_material(
        Box::new(Lambertian::new(Color::from_u8(35, 21, 105)))
    ));
    world.add(Object::new(Shape::Cube).with_transform(
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
        Box::new(Lambertian::new(Color::from_u8(50, 50, 70)))
    ));

    let canvas = render_scene(&world, &camera, &config);

    let outpath = format!("{}.ppm", config.outfile);
    std::fs::write(outpath, canvas.to_ppm())
        .expect("Unable to write file");

    Ok(())
}
