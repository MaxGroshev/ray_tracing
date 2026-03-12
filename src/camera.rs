use std::io::stdout;
use rand::Rng;

use crate::{color::{Color, write_color}, hitable::{HitRecord, Hitable}, ray::Ray, 
            vec3::{Point3, Vec3, random_unit_vector, unit_vector}};


#[derive(Debug, Default)]
pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    pub focal_length: f64,
    pub sampels_per_pixel: i32,

    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    max_rec_level: i32
}

impl Camera {
    pub fn new(width: i32, height:i32, focal:f64) -> Self {
        Self {image_width : width, image_height : height, sampels_per_pixel: 10, 
              max_rec_level:50, focal_length: focal, ..Default::default()}
    }
    fn initialize(&mut self) {
        let viewport_height = 2.0; //TODO: maybe fields?
        let viewport_width = 3.5;
        let camera_center = Point3::from_value(0.0, 0.0, 0.0);

        let viewport_u = Vec3::from_value(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from_value(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = camera_center
                                - Vec3::from_value(0.0, 0.0, self.focal_length)
                                - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }
    
    pub fn render(&mut self, world:&dyn Hitable) {
        self.initialize();

        println!("P3\n{0} {1}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.sampels_per_pixel {
                    let r = self.gen_ray(i, j);
                    pixel_color += self.ray_color(0, &r, world);
                }
                let _ = write_color(&mut stdout(), &(pixel_color/self.sampels_per_pixel as f64));
            }
        }
    }
    fn gen_ray(&self, i:i32, j:i32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
                          + (self.pixel_delta_u * (i as f64 + offset.x()))
                          + (self.pixel_delta_v * (j as f64 + offset.y()));

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::from_vec(ray_origin, ray_direction);
    }
    fn sample_square() -> Vec3 {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(0.0..1.0);
        let random_y = rng.gen_range(0.0..1.0);
        return Vec3::from_value(random_x - 0.5, random_y - 0.5, 0.0);
    }  
    fn ray_color(&self, rec_level: i32 ,r: &Ray, world:&dyn Hitable) -> Color {
        if rec_level >= self.max_rec_level {
            Color::from_value(0.0,0.0,0.0);
        }
        let mut hit_rec = HitRecord::default(); 
        if world.hit(r, &(0.001..f64::INFINITY), &mut hit_rec) {
            let direction = hit_rec.normal + random_unit_vector();
            return 0.7 * self.ray_color(rec_level+1 , &Ray::from_vec(hit_rec.p, direction), world);
            // return (hit_rec.normal + Color::from_value(1.0,1.0,1.0)) * 0.5;
        }
        let unit_direction = unit_vector(&r.direction());
        let  a = 0.5*(unit_direction.y() + 1.0);
        return Color::from_value(1.0, 1.0, 1.0) * (1.0-a) +
               Color::from_value(0.5, 0.7, 1.0) * a;
    }
} 