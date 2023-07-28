use crate::hit::HitData;
use crate::rays::Ray;
use crate::vectors::Vec3;
pub trait Scatterable {
    fn scatter(&self,r:Ray,hit_data:HitData) -> Option<(Ray,Vec3)>;
}
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}
impl Material {
    pub fn scatter(&self, r:Ray,hit:HitData) -> Option<(Ray,Vec3)>{
        match self {
            Material::Lambertian(l) => l.scatter(r,hit),
            Material::Metal(m) => m.scatter(r,hit)
        }
    }
    pub fn new_lambertian(albedo:Vec3) -> Material {
        let lambertian = Lambertian {
            albedo,
        };
        Material::Lambertian(lambertian)
    }
    pub fn new_metal(albedo:Vec3) -> Material {
        let metal = Metal {
            albedo,
        };
        Material::Metal(metal)
    }
}
pub struct Lambertian {
    pub albedo:Vec3
}
impl Scatterable for Lambertian {
    fn scatter(&self,_r:Ray,hit:HitData) -> Option<(Ray,Vec3)> {
        let mut target = hit.point + hit.normal + Vec3::unit_vector(Vec3::rand_in_unit_sphere());
        let diff = target - hit.point;
        let sml = 1e-9;
        if diff.x.abs()<sml && diff.y.abs()<sml && diff.z.abs()<sml {
            target = hit.point + hit.normal;
        }
        let reflected = Ray::new(hit.point,target - hit.point);
        Some((reflected,self.albedo))
    }
}
pub struct Metal {
    pub albedo:Vec3
}
impl Scatterable for Metal {
    fn scatter(&self,r:Ray,hit:HitData) -> Option<(Ray,Vec3)> {
        let reflected = r.dir - (hit.normal * Vec3::dot(r.dir,hit.normal)) * 2.0;
        if Vec3::dot(reflected, hit.normal) > 0.0 {
            let out = Ray::new(hit.point,reflected);
            return Some((out,self.albedo));
        }
        None
    }
}
