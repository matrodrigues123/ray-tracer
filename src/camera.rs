use crate::{vector3::Vector3, ray::Ray};
pub struct Camera {
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left_corner: Vector3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vector3::new(0.0,0.0,focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        let dir = self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin;

        Ray::new(self.origin, dir)
    }
}