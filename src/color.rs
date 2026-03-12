use crate::vec3;
use std::io::{Write, stdout};
use vec3::Vec3;

pub type Color = Vec3;

fn linear_to_gamma(e:f64) -> f64 {
    let g = if e > 0.0 { f64::sqrt(e) } else { 0.0 };
    return g;
}

pub fn write_color<W: Write>(writer: &mut W, color: &Color) -> std::io::Result<()> {
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());

    let rbyte = (255.999 * r) as i32;
    let gbyte = (255.999 * g) as i32;
    let bbyte = (255.999 * b) as i32;
    write!(writer, "{} {} {}\n", rbyte, gbyte, bbyte)
}