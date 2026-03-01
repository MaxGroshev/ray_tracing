use crate::vec3;
use std::io::{Write, stdout};
use vec3::Vec3;

pub fn write_color<W: Write>(writer: &mut W, color: &Vec3) -> std::io::Result<()> {
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;
    write!(writer, "{} {} {}\n", rbyte, gbyte, bbyte)
}