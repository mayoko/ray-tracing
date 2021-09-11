use crate::vec3::{Point3, Vec3, dot};
use crate::ray::Ray;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Box<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: &Point3, outward_normal: &Vec3, mat: &Box<dyn Material>, t: f32, direction: &Vec3) -> Self {
        let front_face = dot(direction, outward_normal) < 0.0;
        let normal = if front_face { *outward_normal } else { -*outward_normal };

        HitRecord { p: *p, normal: normal, mat: mat.clone(), t: t, front_face: front_face }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
