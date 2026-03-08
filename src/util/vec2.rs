use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

fn vec2_new(x: f32, y: f32) -> Vec2 {
    Vec2 { x: x, y: y }
}

impl Vec2 {
    #[inline]
    pub fn dot(&self, v: &Self) -> f32 {
        (self.x * v.x) + (self.y * v.y)
    }

    #[inline]
    pub fn sq_mag(&self) -> f32 {
        self.dot(self)
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
    pub fn clamp(&self, min: f32, max: f32) -> Vec2 {
        Vec2 {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, v: Self) -> Self {
        Vec2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, v: Self) -> Self {
        Vec2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, s: f32) -> Self {
        Vec2 {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    #[inline]
    fn div(self, s: f32) -> Self {
        Vec2 {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl Neg for Vec2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}
