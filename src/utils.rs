use crate::{vector3::Vector3, ray::Ray, hittable::{World, HitRecord, Hittable, Shape}, color::RGBColor, material::{LightReaction, Material}};
use fastrand::Rng;

pub fn clamp(x: f64, min: f64, max: f64) -> f64{
    if x < min {
        return min
    }
    if x > max {
        return max
    }
    return x
}

fn random_vec(min: f64, max: f64, rng: &Rng) -> Vector3 {
    Vector3::new(
        min + (max - min) * rng.f64(),
        min + (max - min) * rng.f64(),
        min + (max - min) * rng.f64(),
    )
}

pub fn random_vec_in_unit_sphere(rng: &Rng) -> Vector3 {
    loop {
        let p = random_vec(-1.0, 1.0, rng);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_vec_in_unit_disk(rng: &Rng) -> Vector3 {
    loop {
        let mut p = random_vec(-1.0, 1.0, rng);
        p.z = 0.0;
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn ray_color(ray: &Ray, world: &World, depth: i32, rng: &Rng) -> RGBColor {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return RGBColor::new(0.0, 0.0, 0.0)
    }

    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(rec) => {
            match rec.material.scatter(rng, ray, &rec) {
                Some(scattered_ray) => {
                    return rec.material.attenuation() * ray_color(&scattered_ray, world, depth - 1, rng)
                },
                None => return RGBColor::new(0.0,0.0,0.0),
            }
        },
        None => {
            let unit_direction = ray.direction.unit();
            let t = 0.5*(unit_direction.y + 1.0);

            let start_value = RGBColor::new(1.0, 1.0, 1.0); // white
            let target_value = RGBColor::new(0.5, 0.7, 1.0); // blue

            start_value*(1.0 - t) + target_value * t
        },
    }

    
}

pub fn random_scene(rng: &Rng) -> World {
    let mut world = World::new();
    let ground_material = Material::Lambertian(RGBColor::new(0.5, 0.5, 0.5));

    world.add(Shape::Sphere { 
        center: Vector3::new(0.0, -1000.0, 0.0), 
        radius: 1000.0, 
        material: ground_material.clone() 
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.f64();
            let center = Vector3::new(
                a as f64 + 0.9*rng.f64(), 0.2, b as f64 + 0.9*rng.f64()
            );

            if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = RGBColor::random(rng) * RGBColor::random(rng);
                    let sphere_material = Material::Lambertian(albedo);
                    world.add(Shape::Sphere { 
                        center,
                        radius: 0.2,
                        material: sphere_material 
                    });
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = RGBColor::random_interval(0.5, 1.0, rng);
                    let fuzz = rng.f64()*0.5;

                    let sphere_material = Material::Metal(albedo, fuzz);

                    world.add(Shape::Sphere { 
                            center,
                            radius: 0.2,
                            material: sphere_material 
                    });
                } else {
                    // glass
                    let sphere_material = Material::Dielectric(RGBColor::new(1.0, 1.0, 1.0), 1.5);
                    world.add(Shape::Sphere { 
                        center,
                        radius: 0.2,
                        material: sphere_material 
                    });
                }
            }
        }
    }

    let material1 = Material::Dielectric(RGBColor::new(1.0, 1.0, 1.0), 1.5);
    world.add(Shape::Sphere { 
        center: Vector3::new(0.0, 1.0, 0.0), 
        radius: 1.0, 
        material: material1 
    });

    let material2 = Material::Lambertian(RGBColor::new(0.4, 0.2, 0.1));
    world.add(Shape::Sphere { 
        center: Vector3::new(-4.0, 1.0, 0.0), 
        radius: 1.0, 
        material: material2 
    });

    let material3 = Material::Metal(RGBColor::new(0.7, 0.6, 0.5), 0.0);
    world.add(Shape::Sphere { 
        center: Vector3::new(4.0, 1.0, 0.0), 
        radius: 1.0, 
        material: material3
    });

    return world;

    
}