use crate::{
    ray::Ray,
    intersection::Intersection,
    colors::Color,
    tuples::Tuple, normal::{reflect, refract},
};
use rand::prelude::*;

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hit: &Intersection) -> Option<Scattered>;
}

impl Default for Box<dyn Material> {
    fn default() -> Box<dyn Material> {
        Box::new(Lambertian::new(Color::new(0.9, 0.9, 0.9)))
    }
}

#[derive(Debug)]
pub struct Scattered {
    pub attenuation: Color,
    pub ray: Ray,
}

impl Scattered {
    pub fn new(attenuation: Color, ray: Ray) -> Scattered {
        Scattered { attenuation, ray }
    }
}


#[derive(Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Intersection) -> Option<Scattered> {
        let hit_position = ray.position(hit.t);
        let normal = hit.object.normal_at(hit_position);
        let mut rng = rand::thread_rng();
        let rand_vec = Tuple::random_vector_in_unit_sphere(&mut rng).unit();
        let new_direction = match rand_vec == -normal {
            false => normal + rand_vec,
            true => normal
        };
        Some(Scattered::new(
            self.albedo,
            Ray::new(hit_position, new_direction)
        ))
    }
}


#[derive(Debug)]
pub struct Metallic {
    albedo: Color,
    fuzz: f64,
}

impl Metallic {
    pub fn new(albedo: Color, fuzz: f64) -> Metallic {
        Metallic {albedo, fuzz}
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, hit: &Intersection) -> Option<Scattered> {
        let position = ray.position(hit.t);
        let normal = hit.object.normal_at(position);
        let reflected = reflect(ray.direction, normal);
        let mut rng = rand::thread_rng();
        let fuzz = Tuple::random_vector_in_unit_sphere(&mut rng) * self.fuzz;
        Some(Scattered::new(
            self.albedo,
            Ray::new(position, reflected + fuzz),
        ))
    }
}


#[derive(Debug)]
pub struct Dielectric {
    pub refractive_index: f64
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric { refractive_index }
    }

    fn schlick_reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = (1. - refractive_index) / (1. + refractive_index);
        let r0 = r0 * r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Intersection) -> Option<Scattered> {
        let position = ray.position(hit.t);
        let normal = hit.object.normal_at(position);
        // Determine if we're going into or out of the material
        // and set refractive ratio and normal accordingly.
        let into_material = normal.dot(&ray.direction) < 0.;
        let nfrom_over_nto = match into_material {
            true => 1./self.refractive_index,
            false => self.refractive_index,
        };
        let normal = if into_material {normal} else {-normal};

        let incoming = ray.direction.unit();
        let cos_theta = -incoming.dot(&normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = nfrom_over_nto * sin_theta > 1.;

        let mut rng = rand::thread_rng();
        let should_reflect = Dielectric::schlick_reflectance(
            cos_theta, nfrom_over_nto) > rng.gen_range(0.0..1.0);
        let direction = if cannot_refract || should_reflect {
            reflect(incoming, normal)
        } else {
            refract(&incoming, normal, nfrom_over_nto)
        };

        Some(Scattered::new(
            Color::new(1., 1., 1.),
            Ray::new(position, direction),
        ))
    }
}