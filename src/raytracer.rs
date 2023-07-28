use crate::color;
use crate::camera::Camera;
use crate::vectors::Vec3;
use crate::rays::Ray;
use crate::sphere::Sphere;
use crate::hit::{HittableList,Hittable};
use crate::materials::Material;

use std::io::{self, Write};
use rand::Rng;


fn ray_color(r: Ray, world:&HittableList, depth:i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0,0.0,0.0);
    }
    if let Some(hit) = world.hit(r,0.001,100000.0){
        if let Some(scattered) = hit.mat_ref.scatter(r,hit) {
            return ray_color(scattered.0,world,depth-1) * scattered.1;
        } else {
            return Vec3::new(0.0,0.0,0.0);
        }
    }
    let normalized_dir: Vec3 = Vec3::unit_vector(r.dir);
    let t = 0.5 * (normalized_dir.y + 1.0);
    Vec3::new(1.0,1.0,1.0) * (1.0 - t) + Vec3::new(0.5,0.7,1.0) * t
}
pub fn run() {
    const WINDOW_ASPECT_RATIO:f64 = 16.0/9.0;
    const WINDOW_WIDTH:u16 = 400;
    const WINDOW_HEIGHT:u16 = (WINDOW_WIDTH as f64/WINDOW_ASPECT_RATIO) as u16;
    let cam = Camera::new();
    let mut world = HittableList::new();

    let material_ground = Material::new_lambertian(Vec3::new(0.8,0.8,0.0));
    let material_center = Material::new_lambertian(Vec3::new(0.7,0.3,0.3));
    let material_left = Material::new_metal(Vec3::new(0.8,0.8,0.8));
    let material_right = Material::new_metal(Vec3::new(0.8,0.6,0.2));

    world.push_sphere(Sphere::new(Vec3::new(0.0,-100.5,1.0),100.0,material_ground));
    world.push_sphere(Sphere::new(Vec3::new(0.0,0.0,1.0),0.5,material_center));
    world.push_sphere(Sphere::new(Vec3::new(-1.0,0.0,1.0),0.5,material_left));
    world.push_sphere(Sphere::new(Vec3::new(1.0,0.0,1.0),0.5,material_right));

    let samples = 1000;
    let depth = 50;
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n255\n", WINDOW_WIDTH, WINDOW_HEIGHT);
    for j in (0..WINDOW_HEIGHT).rev() {
        if j < 10 {
            eprint!("\rLines Remaining:   {}", j);
        } else if j < 100 {
            eprint!("\rLines Remaining:  {}", j);
        } else {
            eprint!("\rLines Remaining: {}", j);
        }
        let mut stderr = io::stderr();
        let _ = stderr.flush();

        for i in 0..WINDOW_WIDTH {
            let mut color = Vec3::new(0.0,0.0,0.0);
            for _ in 0..samples {
                let rand_num:f64 = rng.gen::<f64>();
                let rand_num_2:f64 = rng.gen::<f64>();
                let u = (i as f64 + rand_num)/(WINDOW_WIDTH-1) as f64;
                let v = (j as f64 + rand_num_2)/(WINDOW_HEIGHT-1) as f64;
                let r = cam.get_ray(u,v);
                color = color + ray_color(r,&world,depth);
            }
            color::write_color(color,samples);
        }
    }
    eprintln!("\nDone!");
}
