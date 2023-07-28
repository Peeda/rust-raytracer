use crate::utils;
use std::ops;
use rand::Rng;
#[derive(Copy, Clone,Debug)]
pub struct Vec3 {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vec3 {
    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }
    pub fn magnitude_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }
    pub fn unit_vector(a: Vec3) -> Vec3 {
        let mag = a.magnitude();
        Vec3 {
            x:a.x/mag,
            y:a.y/mag,
            z:a.z/mag,
        }
    }
    pub fn print(a: Vec3) {
        eprintln!("{},{},{}",a.x,a.y,a.z);
    }
    pub fn dot(a:Vec3, b:Vec3) -> f64 {
        a.x*b.x + a.y*b.y + a.z*b.z
    }
    pub fn cross(a:Vec3, b:Vec3) -> Vec3 {
        Vec3 {
            x: a.y*b.z - a.z*b.y,
            y: a.z*b.x - a.x*b.z,
            z: a.x*b.y - a.y*b.x,
        }
    }
    pub fn rand() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            x:rng.gen::<f64>(),
            y:rng.gen::<f64>(),
            z:rng.gen::<f64>(),
        }
    }
    pub fn rand_range(min:f64, max:f64) -> Vec3 {
        Vec3 {
            x:utils::rand_float(min,max),
            y:utils::rand_float(min,max),
            z:utils::rand_float(min,max),
        }
    }
    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let rand_vec = Vec3::rand_range(-1.0,1.0);
            if rand_vec.magnitude_squared() < 1.0 {
                return rand_vec;
            }
        }
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other:Vec3) -> Vec3 {
        Vec3 {
            x:self.x + other.x,
            y:self.y + other.y,
            z:self.z + other.z,
        }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other:Vec3) -> Vec3 {
        Vec3 {
            x:self.x - other.x,
            y:self.y - other.y,
            z:self.z - other.z,
        }
    }
}
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scale:f64) -> Vec3 {
        Vec3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, scale_vec:Vec3) -> Vec3 {
        Vec3 {
            x: self.x * scale_vec.x,
            y: self.y * scale_vec.y,
            z: self.z * scale_vec.z,
        }
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, scale:f64) -> Vec3 {
        Vec3 {
            x: self.x / scale,
            y: self.y / scale,
            z: self.z / scale,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn float_equal(a:f64,b:f64) -> bool {
        if (a-b).abs() < 1e-9 {
            return true;
        }
        false
    }
    fn unique(a:Vec3) -> bool {
        if float_equal(a.x,a.y) || float_equal(a.y,a.z) || float_equal(a.x,a.z) {
            return false;
        }
        true
    }
    #[test]
    fn rand_vector_unique_test() {
        for _ in 0..1000 {
            let rand_vec = Vec3::rand_range(0.0,10.0);
            assert!(unique(rand_vec));
        }
    }
    #[test]
    fn rand_vector_range_test() {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let a = rng.gen_range(0..100) as f64;
            let b = rng.gen_range(100..200) as f64;
            let v = Vec3::rand_range(a,b);
            assert!(v.x>a && v.y>a && v.z>a && v.x<b && v.y<b && v.z<b);
        }
    }
    #[test]
    fn rand_vector_unit_sphere_test() {
        for _ in 0..1000 {
            let v = Vec3::rand_in_unit_sphere();
            assert!((v.x*v.x + v.y*v.y + v.z*v.z) < 1.0);
        }
    }
}
