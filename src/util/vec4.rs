use crate::util::vec3::*;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    #[inline]
    pub fn dot(&self, v: Self) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z) + (self.w * v.w)
    }

    #[inline]
    pub fn sq_mag(&self) -> f32 {
        self.dot(*self)
    }

    #[inline]
    pub fn mag(&self) -> f32 {
        self.sq_mag().sqrt()
    }

    #[inline]
    pub fn norm(&self) -> Self {
        *self / self.mag()
    }

    #[inline]
    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
            w: self.w.clamp(min, max),
        }
    }

    #[inline]
    pub fn lerp(&self, v: Self, amt: f32) -> Self {
        self.clone() + ((v - self.clone()) * amt)
    }

    #[inline]
    pub fn from_vec3(v: Vec3, w: f32) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: w,
        }
    }

    #[inline]
    pub fn from(arr: [f32; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }
}

impl Add for Vec4 {
    type Output = Self;

    #[inline]
    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
            w: self.w + v.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    #[inline]
    fn sub(self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
            w: self.w - v.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn mul(self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    #[inline]
    fn div(self, s: f32) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
            w: self.w / s,
        }
    }
}

impl Neg for Vec4 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray3 {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray3 {
    #[inline]
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Self { origin: o, dir: d }
    }
}
