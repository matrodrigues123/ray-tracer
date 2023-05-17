use crate::{color::RGBColor, ray::Ray, hittable::HitRecord, utils::random_vec_in_unit_sphere};
use fastrand::Rng;

pub trait LightReaction {
    fn scatter(&self, rng: &Rng, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;
}
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(RGBColor),
    Metal(RGBColor, f64),
}

impl Material {
    pub fn attenuation(&self) -> RGBColor {
        match self {
            Material::Lambertian(attenuation) => *attenuation,
            Material::Metal (attenuation, _fuzz) => *attenuation,
        }
    }
}


impl LightReaction for Material {
    fn scatter(&self, rng: &Rng, r_in: &Ray, rec: &HitRecord) -> Option<Ray> {
        match self {
            Material::Lambertian(_) => {
                // lambertian material always scatters the ray and attenuate by its reflectance
                let mut scatter_direction = rec.normal + random_vec_in_unit_sphere(rng).unit();

                // deal with degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                let scattered = Ray::new(rec.point, scatter_direction);
                return Some(scattered)
                
            },
            Material::Metal(_, fuzz) => {
                // the ray isnt randomly scattered, but is reflected
                let reflected = r_in.direction.unit().reflect(rec.normal);
                let scattered = Ray::new(rec.point, reflected + random_vec_in_unit_sphere(rng)*(*fuzz));

                if scattered.direction.dot(rec.normal) > 0.0 {
                    return Some(scattered)    
                }
                else {
                    return None
                }
                
                
            }
            
        }
    }
}
