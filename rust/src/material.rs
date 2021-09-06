use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::vec3::{random_unit_vector, reflect, unit_vector, dot};

pub trait Material: CloneMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Lambertian { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, mut scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Metal {
    pub fn new(a: &Color) -> Self {
        Metal { albedo: *a }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit_vector(&r_in.direction()), &rec.normal);
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

trait CloneMaterial {
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