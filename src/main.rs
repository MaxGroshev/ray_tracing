use std::{io::{Write, stdout}, ops::Range, rc::Rc};

mod vec3;
use vec3::{Vec3, Point3, unit_vector};
use log::info;
mod color;
use color::{Color, write_color};
mod ray;
use ray::Ray;

mod hitable;
use hitable::{Hitable, Sphere, HitableList, HitRecord};

mod utils;
use utils::utils::init_rt;

fn ray_color(r: &Ray, world:&dyn Hitable) -> Color {
    let mut hit_rec = HitRecord::default(); 
    if world.hit(r, &(0.0..f64::INFINITY), &mut hit_rec) {
        return (hit_rec.normal + Color::from_value(1.0,1.0,1.0)) * 0.5;
    }
    let unit_direction = unit_vector(&r.direction());
    let  a = 0.5*(unit_direction.y() + 1.0);
    return Color::from_value(1.0, 1.0, 1.0) * (1.0-a) +
           Color::from_value(0.5, 0.7, 1.0) * a;
}

fn main() {
    init_rt();

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 700;
    let image_height: i32 = 400;
    
    let mut world = HitableList::default();
    world.add(Rc::new(Sphere::new(
            Point3::from_value(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(
            Point3::from_value(0.0,-100.5,-1.0), 100.0)));

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = 3.5;
    let camera_center = Point3::from_value(0.0, 0.0, 0.0);

    let viewport_u = Vec3::from_value(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from_value(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center
                            - Vec3::from_value(0.0, 0.0, focal_length)
                            - viewport_u/2.0 - viewport_v/2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{image_width} {image_height}\n255\n");
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::from_vec(camera_center, ray_direction);

            let pixel_color = ray_color(&r, &world);
            let _ = write_color(&mut stdout(), &pixel_color);
        }
    }
    
    // return 1;
}