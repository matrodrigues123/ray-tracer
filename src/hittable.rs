use crate::color::RGBColor;
use crate::material::Material;

use crate::vector3::Vector3;
use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        // returns a default hitrecord, to be changed
        HitRecord {
            point: Vector3::new(0.0,0.0,0.0),
            normal: Vector3::new(1.0,1.0,1.0),
            material: Material::Lambertian(RGBColor::new(0.0, 0.0, 0.0)),
            t: 0.0,
            front_face: false,
        }
    }
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        // if its negative, the normal is in opposite direction to the ray
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub enum Shape {
    Sphere {radius: f64, center: Vector3, material: Material},
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Shape::Sphere {radius, center, material} => {
                let oc = ray.origin - *center;
                let a = ray.direction.length_squared();
                let half_b = oc.dot(ray.direction);
                let c = oc.length_squared() - radius*radius;
                let discriminant = half_b*half_b - a * c;

                if discriminant < 0.0 {
                    return None
                }

                // find the nearest root in acceptable range
                let sqrtd = discriminant.powf(0.5);
                let mut root = (-half_b - sqrtd) / a;
                if root < t_min || t_max < root {
                    root = (-half_b + sqrtd) / a;
                    if root < t_min || t_max < root {
                        return None
                    }
                }

                let mut hit_rec = HitRecord::new();

                hit_rec.t = root;
                hit_rec.point = ray.at(hit_rec.t);
                let outward_normal = (hit_rec.point - *center) / *radius;
                hit_rec.set_face_normal(ray,outward_normal);
                hit_rec.material = *material;
                
                return Some(hit_rec)
            }
        }
    }

}

pub struct World {
    list: Vec<Shape>,
}

impl World {
    pub fn new() -> Self {
        World { list: Vec::new() }
    }
    pub fn add(&mut self, elem: Shape) {
        self.list.push(elem);
    }
    pub fn clear(&mut self) {
        self.list.clear();
    }
}
impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>{
        // initialized with some default values
        let mut hit_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for shape in &self.list {
            match shape.hit(ray, t_min, closest_so_far) {
                Some(curr_rec) => {
                    hit_anything = true;
                    closest_so_far = curr_rec.t;

                    hit_rec.point = curr_rec.point;
                    hit_rec.normal = curr_rec.normal;
                    hit_rec.t = curr_rec.t;
                    hit_rec.front_face = curr_rec.front_face;
                },
                None => continue,
            }
        }

        if hit_anything {
            Some(hit_rec)
        } else {
            None
        }
    }
}

