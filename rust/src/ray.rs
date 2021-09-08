use crate::vec3::{Vec3, Point3};

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    // pub fn new() -> Self {
    //     Ray {orig: Point3::new(0f32, 0f32, 0f32), dir: Vec3::new(0f32, 0f32, 0f32)}
    // }
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {orig: origin.clone(), dir: direction.clone()}
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}