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

mod material;
use material::{Material, Lambertian, Metal};

mod camera;
use camera::Camera;

mod utils;
use utils::utils::init_rt;

use crate::vec3::{Vec3, random_unit_vector};

fn main() {
    init_rt();
    
    let material_ground: Rc<dyn Material> = Rc::new(Lambertian::new(&Color::from_value(0.8, 0.8, 0.0)));
    let mut  material_center: Rc<dyn Material> = Rc::new(Lambertian::new(&Color::from_value(0.1, 0.2, 0.5)));
    let material_left: Rc<dyn Material>   = Rc::new(Metal::new(&Color::from_value(0.8, 0.8, 0.8), 0.3));
    let material_right: Rc<dyn Material>  = Rc::new(Metal::new(&Color::from_value(0.8, 0.6, 0.2), 1.0));

    let mut world = HitableList::default();
    let s_positions = vec![
        (-0.2, -0.4),
        (0.0, -0.4),
        (0.2, -0.4),
        (0.2, -0.2),
        (0.2, 0.0),
        (0.0, -0.0),
        (-0.2, 0.0),
        (-0.2, 0.2),
        (-0.2, 0.4),
        (0.0, 0.4),
        (0.2, 0.4),

    ];

    for (x, y) in s_positions {
        material_center = Rc::new(Lambertian::new(&random_unit_vector()));
        world.add(Rc::new(Sphere::new(
            Point3::from_value(x, y, -1.0),
            0.1,
            &material_center
        )));
    }

    world.add(Rc::new(Sphere::new(
        Point3::from_value(-1.0,0.0,-1.0), 0.5, &material_left)));
    world.add(Rc::new(Sphere::new(
        Point3::from_value(1.0,0.0,-1.0), 0.5, &material_right)));
    world.add(Rc::new(Sphere::new(
        Point3::from_value(0.0, -100.5, -1.0), 100.0,&material_ground)));
            
    // let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 700;
    let image_height: i32 = 400;
    let focal = 1.0;

    //TODO: mut Camera? weird design
    let mut cam = Camera::new(image_width, image_height, focal);
    cam.render(&world);
    // return 1;
}