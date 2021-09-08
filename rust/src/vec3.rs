use std::ops;
use rand::{thread_rng, Rng};

#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

pub type Point3 = Vec3;

impl Vec3 {
    // methods
    pub fn x(&self) -> f32 {
        return self.x;
    }
    pub fn y(&self) -> f32 {
        return self.y;
    }
    pub fn z(&self) -> f32 {
        return self.z;
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn near_zero(&self) -> bool {
        let eps = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    // static methods
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { x: e0, y: e1, z: e2 }
    }
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Vec3 { x: rng.gen_range(-1.0..1.0), y: rng.gen_range(-1.0..1.0), z: rng.gen_range(-1.0..1.0) }
    }
    pub fn random_range(min: f32, max: f32) -> Self {
        let mut rng = thread_rng();
        Vec3 { x: rng.gen_range(min..max), y: rng.gen_range(min..max), z: rng.gen_range(min..max) }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3{
        x: u[1] * v[2] - u[2] * v[1],
        y: u[2] * v[0] - u[0] * v[2],
        z: u[0] * v[1] - u[1] * v[0]
    }
}

pub fn unit_vector(u: &Vec3) -> Vec3 {
    *u / u.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random();
        if p.length_squared() < 1. {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0*dot(v, n) * *n
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3{ x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Vec3 {
        Vec3{ x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        *self = *self + _rhs;
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: &Vec3) {
        *self = *self + _rhs;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3{ x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: &Vec3) -> Vec3 {
        Vec3{ x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, _rhs: Vec3) {
        *self = *self - _rhs;
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, _rhs: &Vec3) {
        *self = *self - _rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{ x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &f32 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range")
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { x: self.x *_rhs.x(), y: self.y * _rhs.y(), z: self.z * _rhs.z() }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3{ x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3{ x: _rhs.x * self, y: _rhs.y * self, z: _rhs.z * self }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3{ x: self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs }
    }
}