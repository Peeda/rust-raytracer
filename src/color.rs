use crate::vectors::Vec3;

pub fn write_color(color:Vec3) {
    let x = (255.999 * color.x) as u8;
    let y = (255.999 * color.y) as u8;
    let z = (255.999 * color.z) as u8;
    println!("{} {} {}", x,y,z);
}
