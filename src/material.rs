use crate::{color::RGBColor, ray::Ray, hittable::HitRecord, utils::random_vec_in_unit_sphere};
use fastrand::Rng;

trait LightReaction {
    fn scatter(&self, rng: &Rng, r_in: &Ray, rec: &HitRecord, attenuation: RGBColor, scattered: &mut Ray) -> bool;
}
#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(RGBColor),
    Metal(RGBColor),
}

impl LightReaction for Material {
    fn scatter(&self, rng: &Rng, r_in: &Ray, rec: &HitRecord, mut attenuation: RGBColor, scattered: &mut Ray) -> bool {
        match self {
            Material::Lambertian(refl) => {
                // lambertian material always scatters the ray and attenuate by its reflectance
                let mut scatter_direction = rec.normal + random_vec_in_unit_sphere(rng).unit();

                // deal with degenerate scatter direction
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }
                *scattered = Ray::new(rec.point, scatter_direction);
                attenuation = *refl;
                return true
                
            },
            Material::Metal(refl) => {
                // the ray isnt randomly scattered, but is reflected
                let reflected = r_in.direction.unit().reflect(rec.normal);
                *scattered = Ray::new(rec.point, reflected);
                attenuation = *refl;
                
                scattered.direction.dot(rec.normal) > 0.0
            }
            
        }
    }
}
