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
    pub u:Vec3,
    pub v:Vec3,
    pub w:Vec3,
    pub lens_radius:f64,
}
impl Camera {
    pub fn new(from:Vec3, at:Vec3, vup:Vec3, fov:f64, aspect_ratio:f64, aperture:f64,
               focus_dist:f64) -> Camera {
        let theta = fov.to_radians();
        let h = (theta/2.0).tan();

        let viewport_height:f64 = 2.0 * h;
        let viewport_width:f64 = viewport_height * aspect_ratio;
        let focal_length:f64  = 1.0;

        let w = Vec3::unit_vector(from - at);
        let u = Vec3::unit_vector(Vec3::cross(vup,w) * -1.0);
        let v = Vec3::cross(w,u) * -1.0;

        let origin = from;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let bottom_left = origin - horizontal/2.0 - vertical/2.0 - w * focus_dist;
        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            bottom_left,
            u,
            v,
            w,
            lens_radius:aperture/2.0,
        }
    }
    pub fn get_ray(&self, s:f64, t:f64) -> Ray {
        let rand = Vec3::rand_in_unit_disc() * self.lens_radius;
        let offset = self.u * rand.x + self.v * rand.y;
        Ray::new(self.origin + offset, self.bottom_left + self.horizontal*s 
         + self.vertical*t - self.origin - offset)
    }
}
