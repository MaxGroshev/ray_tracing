use core::f64;
use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Index, IndexMut};

// TODO: template?
#[derive(Debug)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }

    pub fn from_array(arr: [f64; 3]) -> Self {
        Self { e: arr }
    }

    pub fn from_value(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: [e1, e2, e3] }
    }

    pub fn length(&self) ->f64 {
        self.squared_length().sqrt()
    }

    pub fn x(&self) ->f64 {
        self.e[0]
    }

    pub fn y(&self) ->f64 {
        self.e[1]
    }

    pub fn z(&self) ->f64 {
        self.e[2]
    }

    // inline vec3 unit_vector(const vec3& v) {
    //     return v / v.length();
    // }
    fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2] 
    }


}

pub fn dot(u: &mut Vec3, v: &mut Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &mut Vec3, v: &mut Vec3) -> Vec3 {
    Vec3::from_value(u.e[1] * v.e[2] - u.e[2] * v.e[1],
                     u.e[2] * v.e[0] - u.e[0] * v.e[2],
                     u.e[0] * v.e[1] - u.e[1] * v.e[0])
}

// --- traits ---

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2]]
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2]]
        }
    }
}

// NOTE: autogeneration of traits impl: pretty convenint
macro_rules! impl_op {
    ($trait:ident, $method:ident) => {
        impl std::ops::$trait for Vec3 {
            fn $method(&mut self, rhs: Self) {
                for i in 0..3 {
                    self.e[i].$method(rhs.e[i]);
                }
            }
        }
    };
}
impl_op!(AddAssign, add_assign);
impl_op!(SubAssign, sub_assign);

impl Neg for &mut Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.e = [-self.e[0], -self.e[1], -self.e[2]];
        self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self::Output {
        Self {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}