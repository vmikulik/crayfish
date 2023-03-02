use crayfish::canvas::Canvas;
use crayfish::colors::Color;
use crayfish::intersection::{intersect, hit};
use crayfish::object::Object;
use crayfish::tuples::Tuple;
use crayfish::matrix::Matrix;
use crayfish::transformations::*;
use crayfish::ray::Ray;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

const DISTANCE: f64 = 4.;
const RADIUS: f64 = 1.;

/// Convert a pixel coordinate to a point on the canvas plane.
/// The canvas plane is centered at the origin, and has a coordinate width and height of 1.
fn plane_coord_from_canvas_coord(x: usize, y: usize) -> (f64, f64) {
    let x = x as f64;
    let y = y as f64;
    let x = (x - WIDTH as f64 / 2.) / WIDTH as f64;
    let y = (y - HEIGHT as f64 / 2.) / HEIGHT as f64;
    (x, y)
}

fn main() {

    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let color = Color::new(1., 0., 0.);

    // let's render a sphere by casting rays from a viewpoint behind the camera
    // to the sphere, passing through the plane of the canvas:
    //
    //                canvas plane
    //                     |
    // ray origin          |         sphere
    //                     |
    //

    let ray_origin = Tuple::point(0., 0., -DISTANCE);
    let sphere_origin = Tuple::point(0., 0., DISTANCE);

    let sphere = Object::new_sphere().with_transform(
        Matrix::identity(4)
            .translate(sphere_origin.x, sphere_origin.y, sphere_origin.z)
            .scale(RADIUS, RADIUS, RADIUS)
    );

    for y_pixel in 0..HEIGHT {
        println!("Rendering row {} of {}", y_pixel, HEIGHT);
        for x_pixel in 0..WIDTH {
            let (x, y) = plane_coord_from_canvas_coord(x_pixel, y_pixel);
            let ray_direction = Tuple::vector(x, y, DISTANCE).unit();
            let ray = Ray::new(ray_origin, ray_direction);
            if hit(intersect(&ray, &sphere).as_slice()).is_some() {
                canvas.write_pixel(x_pixel, y_pixel, color);
            }
        }
    }

    std::fs::write("out.ppm", canvas.to_ppm())
        .expect("Unable to write file");
}
