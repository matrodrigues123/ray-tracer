use crate::{vector3::Vector3, ray::Ray};
pub struct Camera {
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
}

impl Camera {
    pub fn new(lookfrom: Vector3, lookat: Vector3, vup: Vector3, vfov: f64, aspect_ratio: f64) -> Camera {

        let theta = f64::to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);


        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        let dir = self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin;

        Ray::new(self.origin, dir)
    }
}