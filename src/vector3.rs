#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x:f64, y:f64, z:f64) -> Self {
        Vector3 {x, y, z}
    }
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub fn length(&self) -> f64 {
        self.length_squared().powf(0.5)
    }
    pub fn dot(&self, u: Vector3) -> f64 {
        u.x*self.x + u.y*self.y + u.z*self.z
    }
    pub fn cross(&self, u: Vector3) -> Vector3 {
        Vector3 {
            x: self.y*u.z - self.z*u.y,
            y: self.z*u.x - self.x*u.z,
            z: self.x*u.y - self.y*u.x,
        }
    }
    pub fn unit(&self) -> Vector3 {
        self.clone()/self.length()
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
    pub fn reflect(&self, normal: Vector3) -> Vector3 {
        *self - normal*self.dot(normal)*2.0
    }
    pub fn refract(&self, normal: Vector3, snell_refrac_ratio: f64) -> Vector3 {
        let cos_theta = f64::min(-self.dot(normal), 1.0);
        let r_out_perp = (*self + normal * cos_theta) * snell_refrac_ratio;
        let r_out_parallel = normal * (-f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())));

        r_out_perp + r_out_parallel
    }
}


// operation overload
impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl std::ops::Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x/rhs,
            y: self.y/rhs,
            z: self.z/rhs,
        }
    }    
}
impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x*rhs,
            y: self.y*rhs,
            z: self.z*rhs,
        }
    }
}
impl std::ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x*rhs.x,
            y: self.y*rhs.y,
            z: self.z*rhs.z,
        }
    }
}
#[cfg(test)]
mod test {
    use super::Vector3;
    #[test]
    fn length_test() {
        let new_vec = Vector3::new(1.0,2.0,3.0);
        
        assert_eq!(new_vec.length(), (1.0f64.powi(2) + 2.0f64.powi(2) + 3.0f64.powi(2)).powf(0.5))
    }
    #[test]
    fn dot_test() {
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(2.0, 3.0, 4.5);
        let res = 1.0*2.0 + 2.0*3.0 + 3.0*4.5;
        
        assert_eq!(res, vec1.dot(vec2))
    }
    #[test]
    fn cross_test() {
        let vec1 = Vector3::new(4.0, 2.0, -5.0);
        let vec2 = Vector3::new(2.0,-3.0,7.0);
        let res = Vector3::new(-1.0,-38.0,-16.0);
        
        assert_eq!(res, vec1.cross(vec2))
    }
    #[test]
    fn unit_test() {
        let vec1 = Vector3::new(1.0, 1.0, 1.0);
        let res = Vector3::new(1.0/f64::sqrt(3.0),1.0/f64::sqrt(3.0),1.0/f64::sqrt(3.0));
        
        assert_eq!(vec1.unit(), res)
    }
    #[test]
    fn new_test() {
        let new_vec = Vector3::new(1.0,2.0,3.0);
        let cons_vec = Vector3 {
            x:1.0,
            y:2.0,
            z:3.0,
        };
        assert_eq!(new_vec, cons_vec)
    }
    #[test]
    fn add_test(){
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(2.0, 3.0, 4.5);
        let res = Vector3::new(3.0,5.0,7.5);

        assert_eq!(res, vec1 + vec2);
    }
    #[test]
    fn sub_test(){
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(2.0, 3.0, 4.5);
        let res = Vector3::new(-1.0,-1.0,-1.5);

        assert_eq!(res, vec1 - vec2);
    }
    #[test]
    fn div_scalar_test(){
        let vec1 = Vector3::new(2.0, 4.0, 3.0);
        let res = Vector3::new(1.0,2.0,1.5);

        assert_eq!(res, vec1/2.0);
    }
    #[test]
    fn mul_scalar_test(){
        let vec1 = Vector3::new(2.0, 4.0, 3.0);
        let res = Vector3::new(4.0,8.0,6.0);

        assert_eq!(res, vec1*2.0);
    }
    #[test]
    fn mul_vec_test(){
        let vec1 = Vector3::new(2.0, 4.0, 3.0);
        let vec2 = Vector3::new(4.0,8.0,6.0);
        let res = Vector3::new(8.0, 32.0, 18.0);

        assert_eq!(res, vec1*vec2);
    }
    #[test]
    fn neg_test(){
        let vec1 = Vector3::new(2.0, 4.0, 3.0);
        let res = Vector3::new(-2.0,-4.0,-3.0);

        assert_eq!(res, -vec1);
    }
}