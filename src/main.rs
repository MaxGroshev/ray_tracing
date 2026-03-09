use std::{io::{Write, stdout}, ops::Range, rc::Rc};

mod vec3;
use vec3::{Point3};
use log::info;
mod color;
use color::{Color, write_color};
mod ray;
use ray::Ray;

mod hitable;
use hitable::{Hitable, Sphere, HitableList, HitRecord};

mod camera;
use camera::Camera;

mod utils;
use utils::utils::init_rt;

fn main() {
    init_rt();
    
    let mut world = HitableList::default();
    world.add(Rc::new(Sphere::new(
        Point3::from_value(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::from_value(0.0,-100.5,-1.0), 100.0)));
            
    // let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 700;
    let image_height: i32 = 400;
    let focal = 1.0;

    //TODO: mut Camera? weird design
    let mut cam = Camera::new(image_width, image_height, focal);
    cam.render(&world);
    // return 1;
}