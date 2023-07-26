use crate::sphere::Sphere;
use crate::rays::Ray;
use crate::vectors::Vec3;
#[derive(Copy, Clone)]
pub struct HitData {
    pub point:Vec3,
    pub normal:Vec3,
    pub t:f64,
    pub hit_front:bool
}
pub trait Hittable {
    fn hit(&self,r:Ray,t_min:f64,t_max:f64) -> Option<HitData>;
}
enum Hittables {
    Sphere(Sphere),
}
pub struct HittableList {
    list:Vec<Hittables>
}
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            list:Vec::new()
        }
    }
    pub fn push_sphere(&mut self, sphere:Sphere) {
        self.list.push(Hittables::Sphere(sphere));
    }
}
impl Hittable for HittableList {
    fn hit(&self,r:Ray,t_min:f64,t_max:f64) -> Option<HitData> {
        let mut closest_hit:Option<HitData> = None;
        for element in self.list.iter() {
            match element {
                Hittables::Sphere(sphere) => {
                    let curr_hit = sphere.hit(r,t_min,t_max);
                    if let Some(hit) = curr_hit {
                        if closest_hit.is_none() {
                            closest_hit = curr_hit;
                        } else if hit.t < closest_hit.unwrap().t {
                            closest_hit = curr_hit;
                        }
                    }
                }
            }
        }
        closest_hit
    }
}
