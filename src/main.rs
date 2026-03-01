use std::io::{Write, stdout};

mod vec3;
use vec3::Vec3;
use log::info;
mod color;
use color::write_color;
mod utils;
use utils::utils::init_rt;

fn main() {
    init_rt();

    let img_height: i32 = 256;
    let img_width:  i32 = 256;
    println!("P3\n{img_height} {img_width}\n255\n");

    for j in 0..img_height {
        for i in 0..img_width {
            let pixel_color= 
                &Vec3::from_value(f64::from(i)/f64::from(img_width-1), 
            f64::from(j)/f64::from(img_height-1), 
                0.0);
            let _ = write_color(&mut stdout(), pixel_color);
        }
    }
    
    // return 1;
}