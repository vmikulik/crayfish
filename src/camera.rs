use crate::{tuples::Tuple, ray::Ray};

pub struct Camera {
    pub origin: Tuple,
    pub horizontal: Tuple,
    pub vertical: Tuple,
    pub lower_left_corner: Tuple,
}

impl Camera {
    pub fn new(
        lookfrom: Tuple,
        lookat: Tuple,
        aspect_ratio: f64,
        fov_radians: f64,
    ) -> Camera {
        // Figure out the camera plane.
        let up = Tuple::vector(0., 1., 0.);
        let out = (lookat - lookfrom).unit();
        let horizontal = up.cross(&out).unit();
        let vertical = out.cross(&horizontal).unit();

        // Find the viewport size based on aspect ratio and FoV.
        // Assume it is 1 * `out` in front of `lookfrom`.
        let viewport_height = 2. * (fov_radians/2.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let horizontal = horizontal * viewport_width;
        let vertical = vertical * viewport_height;
        let lower_left_corner = lookfrom + out - horizontal * 0.5 - vertical * 0.5;
        Camera {
            origin: lookfrom,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner,
        }
    }

    /// Given x,y in [0, 1], cast ray at the corresponding viewport coordinate.
    pub fn cast_ray(&self, x: f64, y: f64) -> Ray {
        let destination = self.lower_left_corner + self.horizontal * x + self.vertical * y;
        Ray::new(
            self.origin,
            (destination - self.origin).unit(),
        )
    }
}