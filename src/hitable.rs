use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Range, RangeInclusive};
use crate::color::Color;
use crate::material::{Material, Metal};
use crate::vec3::{Point3, Vec3, dot};
use crate::ray::{Ray};

#[derive(Debug, Clone, Copy, Default)]
enum Face {
    #[default]
    Font, 
    Back
}
#[derive(Clone)]
pub struct HitRecord {
    pub mat: Rc<dyn Material>,
    pub p: Point3,
    pub normal: Vec3,
    pub t:f64,
    pub face:Face,
}
impl HitRecord {
    pub fn new () -> Self {
        Self { mat: Rc::new(Metal::default()), p: Point3::new(), normal: Vec3::new(),
        t: 0.0, face : Face::Font }
    }
    fn set_face_normal(&mut self, r:&Ray, outword_normal:&Vec3) {
        let font_face = dot(r.direction(), outword_normal) < 0.0;
        self.face = if font_face { Face::Font } else { Face::Back }
    }
}

//-----------------------------------------------------------------------------

pub trait Hitable {
    fn hit(&self, r:&Ray, interval:&Range<f64>, rec:&mut HitRecord) -> bool; 
    // TODO: Hmmm, no input rec seems useless?
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>
}
impl Sphere {
    pub fn new(center_:Point3, radius_:f64, mat_:&Rc<dyn Material>) -> Self {
        Self { center: center_, radius: f64::max(0.0, radius_), mat: mat_.clone()} 
        //TODO: Increase RC?
    }

}
impl Hitable for Sphere {
    fn hit(&self, r:&Ray, interval:&Range<f64>, rec:&mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.original();
        let a = r.direction().squared_length();
        let h = dot(r.direction(), &oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let discr = h * h - a * c;
        if discr < 0.0 {
            return false;
        }
        let sqrtd = f64::sqrt(discr);
        let mut root = (h - sqrtd) / a;
        if !interval.contains(&root) {
            root = (h + sqrtd) / a;
            if !interval.contains(&root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        
        return true
    }
}

//-----------------------------------------------------------------------------

#[derive(Default)]
pub struct HitableList {
    objects: Vec<Rc<dyn Hitable>>,
}
impl HitableList {
    pub fn add (&mut self, object:Rc<dyn Hitable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) { self.objects.clear(); }
}
impl Hitable for HitableList { // TODO: is there differnce between &mut and Box? 
    fn hit(&self, r:&Ray, interval:&Range<f64>, rec:&mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::new();
        let mut hit_smth: bool  = false;
        let mut innner_interval = interval.clone(); // TODO: looks weird

        for object in &self.objects {
            if object.hit(r, &innner_interval, &mut tmp_rec) {
                hit_smth = true;
                innner_interval.end = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }
        
        return hit_smth
    }
}