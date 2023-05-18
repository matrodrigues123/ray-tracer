use crate::{color::RGBColor, ray::Ray, hittable::HitRecord, utils::random_vec_in_unit_sphere, vector3::Vector3};
use fastrand::Rng;

pub trait LightReaction {
    fn scatter(&self, rng: &Rng, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;
}
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(RGBColor),
    Metal(RGBColor, f64),
    Dielectric(RGBColor, f64),
}

impl Material {
    pub fn attenuation(&self) -> RGBColor {
        match self {
            Material::Lambertian(attenuation) => *attenuation,
            Material::Metal (attenuation, _fuzz) => *attenuation,
            Material::Dielectric(attenuation, _refrac_index) => *attenuation,
        }
    }
    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0-ref_idx)/(1.0+ref_idx);
        r0 = r0*r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
            Material::Dielectric(_, refrac_index) => {
                // always refracts
                let refraction_ratio = match rec.front_face {
                    true => 1.0/(*refrac_index),
                    false => *refrac_index,
                };

                let unit_direction = r_in.direction.unit();

                let cos_theta = f64::min(-unit_direction.dot(rec.normal), 1.0);
                let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction = match cannot_refract || self.reflectance(cos_theta, refraction_ratio) > rng.f64(){
                    true => unit_direction.reflect(rec.normal),
                    false => unit_direction.refract(rec.normal, refraction_ratio)
                };
                

                let scattered = Ray::new(rec.point, direction);
                
                Some(scattered)
            },
            
        }
    }
}
