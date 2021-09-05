use std::ops;

#[derive(PartialEq, Copy, Clone, Default)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

pub type Point3 = Vec3;

impl Vec3 {
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

    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { x: e0, y: e1, z: e2 }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
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