use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

fn vec3_new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x: x, y: y, z: z }
}

impl Vec3 {
    #[inline]
    fn dot(&self, v: &Self) -> f32 {
        (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
    }

    #[inline]
    fn cross(&self, v: &Self) -> Self {
        Vec3 {
            x: (self.y * v.z) - (self.z * v.y),
            y: (self.z * v.x) - (self.x * v.z),
            z: (self.x * v.y) - (self.y * v.x),
        }
    }

    #[inline]
    fn sq_mag(&self) -> f32 {
        self.dot(self)
    }

    #[inline]
    fn mag(&self) -> f32 {
        self.sq_mag().sqrt()
    }

    #[inline]
    fn norm(&self) -> Vec3 {
        *self / self.mag()
    }

    #[inline]
    fn clamp(&self, min: f32, max: f32) -> Vec3 {
        Vec3 {
            x: if self.x < min {
                min
            } else {
                if self.x > max { max } else { self.x }
            },
            y: if self.y < min {
                min
            } else {
                if self.y > max { max } else { self.y }
            },
            z: if self.z < min {
                min
            } else {
                if self.z > max { max } else { self.z }
            },
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, v: Self) -> Self {
        Vec3 {
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
        Vec3 {
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
        Vec3 {
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
        Vec3 {
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

fn ray3_new(o: &Vec3, d: &Vec3) -> Ray3 {
    Ray3 {
        origin: o.clone(),
        dir: d.clone(),
    }
}
