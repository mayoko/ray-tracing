use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::utils::degrees_to_radians;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        vfov: f32, // vertical field-of-view in degrees
        aspect_ratio: f32
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(&self.origin, &(self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin))
    }
}