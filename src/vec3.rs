use std::ops::{Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Index, IndexMut, Div, RangeInclusive};
use rand::Rng;

use crate::vec3;

// TODO: template?
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3]
}

pub type Point3 = Vec3;

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
    
    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2] 
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (f64::abs(self.e[0]) < s) && (f64::abs(self.e[1]) < s) && (f64::abs(self.e[2]) < s)
    }

}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
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

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::from_array([self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2]])
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

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, t: Vec3) -> Self::Output {
        Self {
            e: [self.e[0] * t.e[0], self.e[1] * t.e[1], self.e[2] * t.e[2]]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::from_value(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z(),
        )
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3::from_value(
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z(),
        )
    }
}


impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Self::Output {
        Vec3::from_array([self.e[0] * t, self.e[1] * t, self.e[2] * t])
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self::Output {
        self * (1.0/t) 
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, t: f64) -> Self::Output {
        self * (1.0/t) 
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

pub fn unit_vector(v: &Vec3) -> Vec3{
    return v / v.length();
}

fn random(interval:RangeInclusive<f64>) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::from_value(rng.gen_range(interval.clone()),
                        rng.gen_range(interval.clone()),
                        rng.gen_range(interval))
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random(-1.0..=1.0);
        let lensq = p.squared_length();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / f64::sqrt(lensq);
        }
    }
}

fn random_on_hemisphere(normal:&Vec3) -> Vec3 {
    let mut p = random_unit_vector();
    let res = if dot(normal, &p) > 0.0 { p } else { *-(&mut p) };
    res
}

pub fn reflect(v:&Vec3, n:&Vec3) -> Vec3 {
    return *v - 2.0 * (dot(v,n)*n); //TODO: weird with traits
}

//-----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let vec1 = Vec3::from_value(1.0, 2.0, 3.0);
        let vec2 = Vec3::from_array([4.0, 5.0, 6.0]);
        assert_eq!(dot(&vec1, &vec2), 32.0);

    }

    #[test]
    fn test_add() {
        let vec1 = Vec3::from_value(1.0, 2.0, 3.0);
        let vec2 = Vec3::from_array([4.0, 5.0, 6.0]);
        assert_eq!(vec1 + vec2, Vec3::from_array([5.0, 7.0, 9.0]));

    }
}