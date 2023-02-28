use crayfish::canvas::Canvas;
use crayfish::colors::Color;
use crayfish::tuples::Tuple;

const GRAVITATIONAL_ACCELERATION: f64 = 0.01;
const TIMESTEPS: usize = 500;
const SPEED: f64 = 2.;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn main() {
    let mut velocity = Tuple::vector(1., -1., 0.).unit() * SPEED;
    let mut pos = Tuple::point(0., HEIGHT as f64, 0.);
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut color = Color::new(1., 0., 0.);
    for _t in 0..TIMESTEPS {
        if 0. <= pos.x && pos.x < WIDTH as f64 && 0. <= pos.y && pos.y < HEIGHT as f64 {
            canvas.write_pixel(pos.x as usize, pos.y as usize, color);
        }
        pos = pos + &velocity;
        velocity = velocity + Tuple::vector(0., GRAVITATIONAL_ACCELERATION, 0.);
        color = color + Color::new(-0.002, 0.002, 0.);
    }

    std::fs::write("out.ppm", canvas.to_ppm())
        .expect("Unable to write file");
}
