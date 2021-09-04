use std::fmt;
use crate::vec3::Vec3;

pub type Color = Vec3;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {} {}", (255.999 * self.x()) as u8, (255.999 * self.y()) as u8, (255.999 * self.z()) as u8)
    }
}