use std::fs::File;
use std::io::Write;
use rust_ray_tracer::{vector3::Vector3, hittable::{Shape, World}, camera::Camera, color::RGBColor, utils::ray_color, material::Material};

fn main() {
    // Image
    let rng = fastrand::Rng::new();
    rng.seed(7);
    let aspect_ratio = 16.0/9.0;
    let width = 400;
    let height = ((width as f64)/aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut image_file = File::create("image.ppm").expect("Failed to create file");

    // World
    let material_ground = Material::Lambertian( RGBColor::new(0.8, 0.8, 0.0));
    let material_center = Material::Lambertian( RGBColor::new(0.7, 0.3, 0.3));
    let material_left = Material::Metal( RGBColor::new(0.8,0.8,0.8), 0.3);
    let material_right = Material::Metal( RGBColor::new(0.8,0.6,0.2), 1.0);

    let mut world = World::new();
    world.add(Shape::Sphere { radius: 100.0, center: Vector3::new(0.0,-100.5,-1.0), material: material_ground});
    world.add(Shape::Sphere { radius: 0.5, center: Vector3::new(0.0,0.0,-1.0), material: material_center});
    world.add(Shape::Sphere { radius: 0.5, center: Vector3::new(-1.0,0.0,-1.0), material: material_left});
    world.add(Shape::Sphere { radius: 0.5, center: Vector3::new(1.0,0.0,-1.0), material: material_right});

    // Camera
    let cam = Camera::new(aspect_ratio, 2.0, 1.0);

    
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
    
                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth, &rng);
                
            }

            pixel_color.write_color(&mut image_file, samples_per_pixel);
        }
    }
}
