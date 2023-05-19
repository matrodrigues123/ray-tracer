use fastrand::Rng;

use crate::{vector3::Vector3, ray::Ray, utils::{random_vec_in_unit_sphere, random_vec_in_unit_disk}};
pub struct Camera {
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
    lens_radius: f64,
    u: Vector3,
    v: Vector3,
    w: Vector3,
}

impl Camera {
    pub fn new(lookfrom: Vector3, 
               lookat: Vector3, 
               vup: Vector3, 
               vfov: f64, 
               aspect_ratio: f64,
               aperture: f64,
               focus_dist: f64) -> Camera {

        let theta = f64::to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);


        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - w*focus_dist,
            lens_radius: aperture / 2.0,
            u,
            v,
            w
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &Rng) -> Ray{
        let rd = random_vec_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        let dir = self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin;

        Ray::new(self.origin + offset, dir - offset)
    }
}