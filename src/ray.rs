use crate::vec3;
use std::io::{Write, stdout};
use vec3::{Vec3, Point3};

#[derive(PartialEq, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new() -> Self { //TODO: Useless(use macros)?
        Self { orig: Vec3::new(), dir: Point3::new() }
    }

    pub fn from_vec(orig_: Point3, dir_: Vec3) -> Self {
        Self { orig: orig_, dir: dir_ }
    }

    pub fn original(&self) -> Point3 { self.orig } //TODO: &Point3???

    pub fn direction(&self) -> &Vec3 { &self.dir }

    pub fn at(&self, t: f64) -> Point3 {
        Point3::from_value(self.orig[0] + self.dir[0] * t, 
                           self.orig[1] + self.dir[1] * t, 
                           self.orig[2] + self.dir[2] * t)
    }
}