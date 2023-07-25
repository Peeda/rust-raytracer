use crate::color;
use crate::vectors::Vec3;
use crate::rays::Ray;
use crate::sphere::Sphere;
use crate::hit::{HittableList,Hittable};
use std::io::{self, Write};


fn ray_color(r: Ray, world:&HittableList) -> Vec3 {
    let hit_data = world.hit(r,0.0,100000.0);
    if let Some(hit) = hit_data {
        let mut corrected_normal = hit.normal;
        corrected_normal.z = -corrected_normal.z;
        return (corrected_normal + Vec3::new(1.0,1.0,1.0)) * 0.5;
    }
    let normalized_dir: Vec3 = Vec3::unit_vector(r.dir);
    let t = 0.5 * (normalized_dir.y + 1.0);
    Vec3::new(1.0,1.0,1.0) * (1.0 - t) + Vec3::new(0.5,0.7,1.0) * t
}
pub fn run() {
    const ASPECT_RATIO:f64 = 16_f64/9_f64;
    const WINDOW_WIDTH:u16 = 400;
    const WINDOW_HEIGHT:u16 = (WINDOW_WIDTH as f64/ASPECT_RATIO) as u16;

    let viewport_height:f64 = 2_f64;
    let viewport_width:f64 = viewport_height * ASPECT_RATIO;
    let focal_length:f64  = 1.0;

    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(viewport_width,0.0,0.0);
    let vertical = Vec3::new(0.0,viewport_height,0.0);
    let bottom_left = Vec3 {
        x: -viewport_width/2.0,
        y: -viewport_height/2.0,
        z: focal_length,
    };

    let mut world = HittableList::new();
    world.push_sphere(Sphere::new(Vec3::new(0.0,0.0,1.0),0.5));
    world.push_sphere(Sphere::new(Vec3::new(0.0,-100.5,1.0),100.0));

    println!("P3\n{} {}\n255\n", WINDOW_WIDTH, WINDOW_HEIGHT);
    for j in (0..WINDOW_HEIGHT).rev() {

        eprint!("\rLines Remaining: {}", j);
        let mut stderr = io::stderr();
        let _ = stderr.flush();

        for i in 0..WINDOW_WIDTH {
            let u = i as f64/(WINDOW_WIDTH-1) as f64;
            let v = j as f64/(WINDOW_HEIGHT-1) as f64;

            let r = Ray::new(origin,bottom_left + horizontal*u + vertical*v - origin);

            color::write_color(ray_color(r,&world));
        }
    }
    eprintln!("\nDone!");
}
