use crate::color;
use crate::camera::Camera;
use crate::vectors::Vec3;
use crate::rays::Ray;
use crate::sphere::Sphere;
use crate::hit::{HittableList,Hittable};
use crate::materials::Material;
use crate::utils;

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
    /*
    const WINDOW_ASPECT_RATIO:f64 = 16.0/9.0;
    const WINDOW_WIDTH:u16 = 400;
    const WINDOW_HEIGHT:u16 = (WINDOW_WIDTH as f64/WINDOW_ASPECT_RATIO) as u16;

    let from = Vec3::new(3.0,3.0,-2.0);
    let at = Vec3::new(0.0,0.0,1.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let focus_dist = (from - at).magnitude();
    let aperture = 2.0;

    let cam = Camera::new(from, at, vup, 20.0, 16.0/9.0, aperture, focus_dist);
    // let mut world = HittableList::new();

        /*
        let r = (3.14159/4.0 as f64).cos();
        let material_left = Material::new_lambertian(Vec3::new(0.0,0.0,1.0));
        let material_right = Material::new_lambertian(Vec3::new(1.0,0.0,0.0));

        world.push_sphere(Sphere::new(Vec3::new(-r,0.0,1.0),r,material_left));
        world.push_sphere(Sphere::new(Vec3::new(r,0.0,1.0),r,material_right));
        */

    let material_ground = Material::new_lambertian(Vec3::new(0.8,0.8,0.0));
    let material_center = Material::new_lambertian(Vec3::new(0.1,0.2,0.5));
    let material_left = Material::new_dielectric(1.5);
    let material_right = Material::new_metal(Vec3::new(0.8,0.6,0.2),0.0);

    let mat_temp = Material::new_dielectric(1.5);

    // world.push_sphere(Sphere::new(Vec3::new(0.0,-100.5,1.0),100.0,material_ground));
    // world.push_sphere(Sphere::new(Vec3::new(0.0,0.0,1.0),0.5,material_center));
    // world.push_sphere(Sphere::new(Vec3::new(-1.0,0.0,1.0),0.5,material_left));
    // world.push_sphere(Sphere::new(Vec3::new(-1.0,0.0,1.0),-0.4,mat_temp));
    // world.push_sphere(Sphere::new(Vec3::new(1.0,0.0,1.0),0.5,material_right));
    */


    const WINDOW_ASPECT_RATIO:f64 = 3.0/2.0;
    const WINDOW_WIDTH:u16 = 1200;
    const WINDOW_HEIGHT:u16 = (WINDOW_WIDTH as f64/WINDOW_ASPECT_RATIO) as u16;

    let from = Vec3::new(13.0,2.0,-3.0);
    let at = Vec3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let world = cover_image();

    let samples = 500;
    let depth = 50;
    let mut rng = rand::thread_rng();

    let cam = Camera::new(from, at, vup, 20.0, WINDOW_ASPECT_RATIO, aperture, focus_dist);

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
fn cover_image() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Material::new_lambertian(Vec3::new(0.5,0.5,0.5));
    world.push_sphere(Sphere::new(Vec3::new(0.0,-1000.0,0.0),1000.0,ground_material));
    let mut rng = rand::thread_rng();

    for i in -11..11 {
        for k in -11..11 {
            let rand = rng.gen::<f64>();
            let center = Vec3::new(i as f64 + 0.9 *rng.gen::<f64>(), 0.2, 
                                   k as f64 + 0.9*rng.gen::<f64>());

            if (center - Vec3::new(4.0,0.2,0.0)).magnitude() > 0.9 {
                if rand < 0.8 {
                    let albedo = Vec3::rand() * Vec3::rand();
                    let mat = Material::new_lambertian(albedo);
                    world.push_sphere(Sphere::new(center,0.2,mat));
                } else if rand < 0.95 {
                    let albedo = Vec3::rand_range(0.5,1.0);
                    let fuzz = utils::rand_float(0.0,0.5);
                    let mat = Material::new_metal(albedo,fuzz);
                    world.push_sphere(Sphere::new(center,0.2,mat));
                } else {
                    let mat = Material::new_dielectric(1.5);
                    world.push_sphere(Sphere::new(center,0.2,mat));
                }
            }
        }
    }
    let mat_1 = Material::new_dielectric(1.5);
    let mat_2 = Material::new_lambertian(Vec3::new(0.4,0.2,0.1));
    let mat_3 = Material::new_metal(Vec3::new(0.7,0.6,0.5),0.0);

    world.push_sphere(Sphere::new(Vec3::new(0.0,1.0,0.0),1.0,mat_1));
    world.push_sphere(Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0,mat_2));
    world.push_sphere(Sphere::new(Vec3::new(4.0,1.0,0.0),1.0,mat_3));

    world
}
