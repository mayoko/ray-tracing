use std::ops;

pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        return self.e[0];
    }
    pub fn y(&self) -> f32 {
        return self.e[1];
    }
    pub fn z(&self) -> f32 {
        return self.e[2];
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }
    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3{ e: [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0]
    ] }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3{ e: [self.e[0] + _rhs.e[0], self.e[1] + _rhs.e[1], self.e[2] + _rhs.e[2]] }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3{ e: [self.e[0] - _rhs.e[0], self.e[1] - _rhs.e[1], self.e[2] - _rhs.e[2]] }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3{e: [-self.e[0], -self.e[1], -self.e[2]]}
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3{ e: [self.e[0] * _rhs, self.e[1] * _rhs, self.e[2] * _rhs]}
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3{ e: [_rhs.e[0] * self, _rhs.e[1] * self, _rhs.e[2] * self]}
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3{ e: [self.e[0] / _rhs, self.e[1] / _rhs, self.e[2] / _rhs]}
    }
}