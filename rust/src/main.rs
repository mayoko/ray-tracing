mod vec3;
mod color;
mod ray;

use vec3::{Vec3, Point3, unit_vector};
use color::Color;
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5*(unit_direction.y() + 1f32);
    (1f32 - t) * Color::new(1f32, 1f32, 1f32) + t * Color::new(0.5f32, 0.7f32, 1f32)
}

fn main() {
    // Image
    let aspect_ratio = 16f32 / 9f32;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f32) / (image_width-1) as f32;
            let v = (j as f32) / (image_height-1) as f32;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let color = ray_color(&r);

            println!("{}", color);
        }
    }
    eprintln!("\nDone.");
}
