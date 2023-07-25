use crate::vectors::Vec3;
use crate::rays::Ray;
use crate::hit::{Hittable,HitData};
pub struct Sphere {
    pub center:Vec3,
    pub radius:f64,
}
impl Sphere {
    pub fn new(center:Vec3, radius:f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self,r:Ray,t_min:f64,t_max:f64) -> Option<HitData> {
        let ac_diff = r.origin - self.center;
        let a = r.dir.magnitude_squared();
        let half_b = Vec3::dot(r.dir, ac_diff);
        let c = ac_diff.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        } else {
            let mut t = (-half_b - discriminant.sqrt())/a;
            if !(t > t_min && t < t_max) {
                t = (-half_b + discriminant.sqrt())/a;
                if !(t > t_min && t < t_max) {
                    return None;
                }
            }
            let mut normal:Vec3 = (r.at(t) - self.center)/self.radius;
            let hit_front:bool = if Vec3::dot(r.dir,normal) > 0.0 {
                normal = -normal;
                false
            } else {
                true
            };
            let out = HitData {
                point:r.at(t),
                normal,
                t,
                hit_front
            };
            return Some(out);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_miss() {
        let sphere = Sphere {
            center:Vec3::new(0.0,0.0,1.0),
            radius:0.5
        };
        let ray = Ray {
            origin:Vec3::new(0.0,0.0,0.0),
            dir:Vec3::new(0.0,1.0,0.0)
        };
        assert!(sphere.hit(ray,-1000.0,1000.0).is_none());
    }
    #[test]
    fn test_hit() {
        let sphere = Sphere {
            center:Vec3::new(0.0,0.0,2.0),
            radius:1.0
        };
        let ray = Ray {
            origin:Vec3::new(0.0,0.0,0.0),
            dir:Vec3::new(0.0,0.0,1.0)
        };
        let out = sphere.hit(ray,-1000.0,1000.0);
        let diff = (out.unwrap().t - 1.0).abs();
        assert!(diff < 0.01);
    }
    #[test]
    fn test_low_t() {
        let sphere = Sphere {
            center:Vec3::new(0.0,0.0,2.0),
            radius:1.0
        };
        let ray = Ray {
            origin:Vec3::new(0.0,0.0,0.0),
            dir:Vec3::new(0.0,0.0,1.0)
        };
        let out = sphere.hit(ray,3.01,200.0);
        assert!(out.is_none());
    }
    #[test]
    fn test_high_t() {
        let sphere = Sphere {
            center:Vec3::new(0.0,0.0,2.0),
            radius:1.0
        };
        let ray = Ray {
            origin:Vec3::new(0.0,0.0,0.0),
            dir:Vec3::new(0.0,0.0,1.0)
        };
        let out = sphere.hit(ray,0.0,0.99);
        assert!(out.is_none());
    }
}
