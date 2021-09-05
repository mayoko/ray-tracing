use std::fmt;
use crate::vec3::Vec3;
use crate::utils::clamp;

pub type Color = Vec3;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {} {}", (255.999 * self.x()) as u8, (255.999 * self.y()) as u8, (255.999 * self.z()) as u8)
    }
}

pub fn scale_color_by_samples(color: &Color, samples_per_pixel: u32) -> Color {
    let scale = 1.0 / samples_per_pixel as f32;
    let r = clamp((color.x() * scale).sqrt(), 0.0, 0.999);
    let g = clamp((color.y() * scale).sqrt(), 0.0, 0.999);
    let b = clamp((color.z() * scale).sqrt(), 0.0, 0.999);

    Color::new(r, g, b)
}