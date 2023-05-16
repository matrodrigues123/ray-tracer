use crate::{vector3::Vector3, ray::Ray, hittable::{World, HitRecord, Hittable}, color::RGBColor};
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

pub fn ray_color(ray: &Ray, world: &World, depth: i32, rng: &Rng) -> RGBColor {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return RGBColor::new(0.0, 0.0, 0.0)
    }

    // initialized with some default values
    let mut rec = HitRecord {
        point: Vector3::new(0.0,0.0,0.0),
        normal: Vector3::new(1.0,1.0,1.0),
        t: 0.0,
        front_face: false,
    };
    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.point + rec.normal + random_vec_in_unit_sphere(rng);


        let rand_ray = Ray::new(rec.point, target - rec.point);

        return ray_color(&rand_ray, &world, depth - 1, rng)*0.5
    }

    let unit_direction = ray.direction.unit();
    let t = 0.5*(unit_direction.y + 1.0);

    let start_value = RGBColor::new(1.0, 1.0, 1.0); // white
    let target_value = RGBColor::new(0.5, 0.7, 1.0); // blue

    start_value*(1.0 - t) + target_value * t
}