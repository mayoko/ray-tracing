mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod utils;
mod camera;

use rand::Rng;

use vec3::{Vec3, Point3, unit_vector, dot};
use color::{Color, scale_color_by_samples};
use ray::Ray;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use sphere::Sphere;
use utils::{PI, INFINITY, degrees_to_radians};
use camera::Camera;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec: HitRecord = Default::default();
    if world.hit(&r, 0.0, INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = unit_vector(&r.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel: u32 = 100;

    // World
    let mut world: HittableList = Default::default();
    world.add(Box::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color_sum = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen_range(0.0..1.0)) / (image_width-1) as f32;
                let v = (j as f32 + rng.gen_range(0.0..1.0)) / (image_height-1) as f32;
                let r = camera.get_ray(u, v);
                pixel_color_sum += ray_color(&r, &world);
            }
            println!("{}", scale_color_by_samples(&pixel_color_sum, samples_per_pixel));
        }
    }
    eprintln!("\nDone.");
}
