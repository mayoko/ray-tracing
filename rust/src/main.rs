mod vec3;
mod color;

use vec3::Vec3;
use color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let color = Color::new((i as f32) / ((image_width-1) as f32), (j as f32) / ((image_height-1) as f32), 0.25);

            println!("{}", color);
        }
    }
    eprintln!("\nDone.");
}
