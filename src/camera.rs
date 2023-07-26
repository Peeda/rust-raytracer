use crate::vectors::Vec3;
use crate::rays::Ray;
pub struct Camera {
    pub aspect_ratio:f64,
    pub viewport_height:f64,
    pub viewport_width:f64,
    pub focal_length:f64,
    pub origin:Vec3,
    pub horizontal:Vec3,
    pub vertical:Vec3,
    pub bottom_left:Vec3,
}
impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio:f64 = 16_f64/9_f64;

        let viewport_height:f64 = 2_f64;
        let viewport_width:f64 = viewport_height * aspect_ratio;
        let focal_length:f64  = 1.0;

        let origin = Vec3::new(0.0,0.0,0.0);
        let horizontal = Vec3::new(viewport_width,0.0,0.0);
        let vertical = Vec3::new(0.0,viewport_height,0.0);
        let bottom_left = Vec3 {
            x: -viewport_width/2.0,
            y: -viewport_height/2.0,
            z: focal_length,
        };
        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            bottom_left
        }
    }
    pub fn get_ray(&self, u:f64, v:f64) -> Ray {
        Ray::new(self.origin,self.bottom_left + self.horizontal*u 
         + self.vertical*v - self.origin)
    }
}
