use crate::hit::HitData;
use crate::rays::Ray;
use crate::vectors::Vec3;
use rand::Rng;
pub trait Scatterable {
    fn scatter(&self,r:Ray,hit_data:HitData) -> Option<(Ray,Vec3)>;
}
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}
impl Material {
    pub fn scatter(&self, r:Ray,hit:HitData) -> Option<(Ray,Vec3)>{
        match self {
            Material::Lambertian(l) => l.scatter(r,hit),
            Material::Metal(m) => m.scatter(r,hit),
            Material::Dielectric(d) => d.scatter(r,hit),
        }
    }
    pub fn new_lambertian(albedo:Vec3) -> Material {
        let lambertian = Lambertian {
            albedo,
        };
        Material::Lambertian(lambertian)
    }
    pub fn new_metal(albedo:Vec3,blur:f64) -> Material {
        let metal = Metal {
            albedo,
            blur
        };
        Material::Metal(metal)
    }
    pub fn new_dielectric(refraction:f64) -> Material {
        let dielectric:Dielectric = Dielectric {
            refraction,
        };
        Material::Dielectric(dielectric)
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
    pub albedo:Vec3,
    pub blur:f64
}
impl Scatterable for Metal {
    fn scatter(&self,r:Ray,hit:HitData) -> Option<(Ray,Vec3)> {
        let reflected = Vec3::reflect(r.dir,hit.normal);
        if Vec3::dot(reflected, hit.normal) > 0.0 {
            let out = Ray::new(hit.point,reflected + Vec3::rand_in_unit_sphere() * self.blur);
            return Some((out,self.albedo));
        }
        None
    }
}
pub struct Dielectric {
    pub refraction:f64,
}
impl Dielectric {
    fn reflectance(cos:f64, ref_idx:f64) -> f64 {
        let mut r = (1.0-ref_idx) / (1.0+ref_idx);
        r = r * r;
        r + (1.0-r) * (1.0-cos).powf(5.0)
    }
}
impl Scatterable for Dielectric {
    fn scatter(&self,r:Ray,hit:HitData) -> Option<(Ray,Vec3)> {
        let refraction_ratio = if hit.hit_front {1.0/self.refraction} else {self.refraction};
        let unit_dir = Vec3::unit_vector(r.dir);
        let cos_theta = if Vec3::dot(-unit_dir,hit.normal) < 1.0 {
            Vec3::dot(-unit_dir,hit.normal)
        } else {
            1.0
        };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let rng = rand::thread_rng;
        let rand_float = rng().gen::<f64>();
        let dir = if refraction_ratio * sin_theta > 1.0 || 
            Dielectric::reflectance(cos_theta,refraction_ratio) > rand_float {
            Vec3::reflect(unit_dir,hit.normal)
        } else {
            Vec3::refract(unit_dir,hit.normal,refraction_ratio)
        };
        Some((Ray::new(hit.point,dir),Vec3::new(1.0,1.0,1.0)))
    }
}
