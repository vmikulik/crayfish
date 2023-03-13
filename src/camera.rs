use crate::{tuples::Tuple, ray::Ray};

pub struct Camera {
    pub origin: Tuple,
    pub horizontal: Tuple,
    pub vertical: Tuple,
    pub lower_left_corner: Tuple,
    aperture_radius: f64,
    u_horizontal: Tuple,
    u_vertical: Tuple,
}

impl Camera {
    pub fn new(
        lookfrom: Tuple,
        lookat: Tuple,
        aspect_ratio: f64,
        fov_radians: f64,
        focus_distance: Option<f64>, // if None, use |lookfrom - lookat|
        aperture_radius: f64,
    ) -> Camera {
        // Figure out the camera plane.
        let up = Tuple::vector(0., 1., 0.);
        let u_out = (lookat - lookfrom).unit();
        let u_horizontal = up.cross(&u_out).unit();
        let u_vertical = u_out.cross(&u_horizontal).unit();

        // Find the viewport size based on aspect ratio and FoV.
        // Assume it is 1 * `out` in front of `lookfrom`.
        let viewport_height = 2. * (fov_radians/2.).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let focus_distance = focus_distance.unwrap_or_else(||
            (lookfrom - lookat).magnitude());
        let horizontal = u_horizontal * viewport_width * focus_distance;
        let vertical = u_vertical * viewport_height * focus_distance;
        let lower_left_corner = lookfrom + u_out * focus_distance - horizontal * 0.5 - vertical * 0.5;
        Camera {
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            aperture_radius,
            u_horizontal,
            u_vertical,
        }
    }

    /// Given x,y in [0, 1], cast ray at the corresponding viewport coordinate.
    pub fn cast_ray(&self, x: f64, y: f64) -> Ray {
        // We pick a random point on the aperture, and pretend the light goes through that.
        let r = Tuple::random_in_unit_disc();
        let offset = (self.u_horizontal * r.x + self.u_vertical * r.y) * self.aperture_radius;

        let destination = self.lower_left_corner + self.horizontal * x + self.vertical * y;
        Ray::new(
            self.origin + offset,
            destination - self.origin - offset,
        )
    }
}