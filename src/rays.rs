use crate::vectors::Vec3;
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin:Vec3,
    pub dir:Vec3
}
impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
    pub fn new(origin:Vec3, dir:Vec3) -> Ray {
        Ray {
            origin,
            dir,
        }
    }
}
