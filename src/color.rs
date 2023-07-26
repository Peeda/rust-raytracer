use crate::vectors::Vec3;
use crate::utils;

pub fn write_color(color:Vec3, samples:i32) {
    let (mut r,mut g,mut b) = (color.x,color.y,color.z);
    let scale:f64 = 1f64/samples as f64;
    (r,g,b) = (r*scale,g*scale,b*scale);
    r = utils::clamp(r,0.0,0.9999);
    g = utils::clamp(g,0.0,0.9999);
    b = utils::clamp(b,0.0,0.9999);
    let x = (256.0 * r) as u8;
    let y = (256.0 * g) as u8;
    let z = (256.0 * b) as u8;
    println!("{} {} {}", x,y,z);
}
