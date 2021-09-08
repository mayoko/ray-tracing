use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vec3::{random_unit_vector, reflect, unit_vector, dot, random_in_unit_sphere};

pub trait Material: CloneMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray);
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

#[derive(Clone)]
pub struct Dielectric {
    ir: f32, // index of refraction
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Lambertian { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(&rec.p, &scatter_direction);

        (true, attenuation, scattered)
    }
}

impl Metal {
    pub fn new(a: &Color, f: f32) -> Self {
        Metal { albedo: *a, fuzz: if f < 1.0 { f } else { 1.0 } }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(&rec.p, &(reflected + self.fuzz*random_in_unit_sphere()));

        (dot(&scattered.direction(), &rec.normal) > 0.0, attenuation, scattered)
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Dielectric { ir: index_of_refraction }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> (bool, Color, Ray) {
        (true, Color::new(1.0, 1.0, 1.0), Ray::new(&rec.p, &r_in.direction()))
    }
}

pub trait CloneMaterial {
    fn clone_material<'a>(&self) -> Box<dyn Material>;
}

impl<T> CloneMaterial for T
where
    T: Material + Clone + 'static,
{
    fn clone_material(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Self {
        self.clone_material()
    }
}