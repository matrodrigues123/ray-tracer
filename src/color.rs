use std::fs::File;
use std::io::Write;
use fastrand::Rng;

use crate::utils::{clamp};
#[derive(Debug,PartialEq,Clone,Copy)]
pub struct RGBColor {
    r: f64,
    g: f64,
    b: f64,
}



impl RGBColor {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        RGBColor {r, g, b}
    }
    pub fn write_color(&self, f: &mut File, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;

        let r= (256.0 * clamp(f64::sqrt(self.r * scale), 0.0, 0.999)).round() as i32;
        let g = (256.0 * clamp(f64::sqrt(self.g * scale), 0.0, 0.999)).round() as i32;
        let b = (256.0 * clamp(f64::sqrt(self.b * scale), 0.0, 0.999)).round() as i32;


        write!(f, "{} {} {}\n", r, g, b).expect("failed to write to data");
    }

    pub fn random(rng: &Rng) -> RGBColor{
        RGBColor::new(rng.f64(), rng.f64(), rng.f64())
    }
    pub fn random_interval(min: f64, max: f64, rng: &Rng) -> RGBColor {
        RGBColor::new(
            min + (max - min) * rng.f64(),
            min + (max - min) * rng.f64(),
            min + (max - min) * rng.f64(),
        )
    }
}

impl std::ops::Mul<f64> for RGBColor {
    type Output = RGBColor;
    fn mul(self, rhs: f64) -> Self::Output {
        RGBColor {
            r: self.r*rhs,
            g: self.g*rhs,
            b: self.b*rhs,
        }
    }
}
impl std::ops::Mul<RGBColor> for RGBColor {
    type Output = RGBColor;
    fn mul(self, rhs: RGBColor) -> Self::Output {
        RGBColor {
            r: self.r*rhs.r,
            g: self.g*rhs.g,
            b: self.b*rhs.b,
        }
    }
}
impl std::ops::Add<RGBColor> for RGBColor {
    type Output = RGBColor;
    fn add(self, rhs: RGBColor) -> Self::Output {
        RGBColor {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}
