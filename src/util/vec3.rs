use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    #[inline]
    pub fn dot(&self, v: Self) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    #[inline]
    pub fn cross(&self, v: Self) -> Self {
        Self {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
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
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, v: Self) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, v: Self) -> Self {
        Self {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, s: f32) -> Self {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, s: f32) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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
