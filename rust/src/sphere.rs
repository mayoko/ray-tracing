use crate::vec3::{Point3, dot};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Option<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(cen: &Point3, r: f32, m: &Option<Box<dyn Material>>) -> Self {
        Sphere { center: *cen, radius: r, mat: m.clone() }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec: HitRecord = Default::default();

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(&r, &outward_normal);
        rec.mat = self.mat.clone();

        Some(rec)
    }
}