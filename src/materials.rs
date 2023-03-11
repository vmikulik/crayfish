use crate::{
    ray::Ray,
    intersection::Intersection,
    colors::Color,
    tuples::Tuple, normal::reflect,
};

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
        Some(Scattered {
            attenuation: self.albedo,
            ray: Ray::new(hit_position, new_direction)
        })
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
        Some(Scattered {
            attenuation: self.albedo,
            ray: Ray::new(position, reflected + fuzz),
        })
    }
}