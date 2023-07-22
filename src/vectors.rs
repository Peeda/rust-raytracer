use std::ops;
#[derive(Copy, Clone)]
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
