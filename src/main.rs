use std::{fs::File, f32::consts::PI};
use std::io::Write;
use rust_ray_tracer::utils::random_scene;
use rust_ray_tracer::{vector3::Vector3, hittable::{Shape, World}, camera::Camera, color::RGBColor, utils::ray_color, material::Material};

fn main() {
    // Image
    let rng = fastrand::Rng::new();
    rng.seed(10);
    let aspect_ratio = 3.0/2.0;
    let width = 400;
    let height = ((width as f64)/aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut image_file = File::create("image.ppm").expect("Failed to create file");

    // World
    let world = random_scene(&rng);

    // Camera
    let cam = Camera::new(
        Vector3::new(8.0,5.0,10.0),
        Vector3::new(0.0,0.0,0.0), 
        Vector3::new(0.0,1.0,0.0),
        20.0, 
        aspect_ratio,
        0.1,
        10.0
    );

    
    // Render

    // Write that colors are in ASCII,
    // that there are 256 columns and 256 rows,
    // and that the max color is 255
    write!(image_file, "P3\n{} {}\n255\n", width, height).expect("Failed to write data");

    for h in (0..height).rev() {
        for w in 0..width {
            let mut pixel_color = RGBColor::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let v = (h as f64 + rng.f64()) / (height - 1) as f64;
                let u = (w as f64 + rng.f64()) / (width - 1) as f64;
    
                let ray = cam.get_ray(u, v, &rng);
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth, &rng);
                
            }

            pixel_color.write_color(&mut image_file, samples_per_pixel);
        }
    }
}
