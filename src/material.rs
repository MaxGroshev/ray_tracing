use crate::{color::Color, hitable::HitRecord, ray::Ray, vec3::{dot, random_unit_vector, reflect, unit_vector}};

pub trait Material {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo_:&Color) -> Self {
        Self { albedo: *albedo_ }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();
        // Bad case
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::from_vec(rec.p, scatter_direction); // TODO: interesting case
        // BAD: scattered = &mut Ray::from_vec(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64
}
impl Metal {
    pub fn new(albedo_:&Color, mut fuzz_:f64) -> Self {
        fuzz_ = if fuzz_ < 1.0 { fuzz_ } else { 0.0 };
        Self { albedo: *albedo_, fuzz: fuzz_ }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in:&Ray, rec:&HitRecord, attenuation:&mut Color, scattered:&mut Ray) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered = Ray::from_vec(rec.p, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction(), &rec.normal) > 0.0
    }
}