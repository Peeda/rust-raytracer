use crate::color;
use crate::vectors::Vec3;
use crate::rays::Ray;
use std::io::{self, Write};


fn hit_sphere(center:Vec3, radius:f64, r:Ray) -> bool {
    let ac_diff = r.origin - center;
    let a = Vec3::dot(r.dir, r.dir);
    let b = 2f64 * Vec3::dot(r.dir, ac_diff);
    let c = Vec3::dot(ac_diff,ac_diff) - radius * radius;
    b*b - 4f64*a*c > 0f64
}
fn ray_color(r: Ray) -> Vec3 {
    let normalized_dir: Vec3 = Vec3::unit_vector(r.dir);
    let t = 0.5 * (normalized_dir.y + 1.0);
    if hit_sphere(Vec3::new(0.0,0.0,1.0),0.5,r) {
        return Vec3::new(255.0,0.0,0.0);
    }
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

    println!("P3\n{} {}\n255\n", WINDOW_WIDTH, WINDOW_HEIGHT);
    for j in (0..WINDOW_HEIGHT).rev() {

        eprint!("\rLines Remaining: {}", j);
        let mut stderr = io::stderr();
        let _ = stderr.flush();

        for i in 0..WINDOW_WIDTH {
            let u = i as f64/(WINDOW_WIDTH-1) as f64;
            let v = j as f64/(WINDOW_HEIGHT-1) as f64;

            let r = Ray::new(origin,bottom_left + horizontal*u + vertical*v - origin);

            color::write_color(ray_color(r));
        }
    }
    eprintln!("\nDone!");
}
