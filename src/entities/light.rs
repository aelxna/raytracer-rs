use crate::util::vec3::*;

#[derive(Debug, Clone)]
pub struct Light {
    pub pos: Vec3,
    pub point: bool,
    pub intensity: f32,
}

impl Light {
    #[inline]
    pub fn new(p: Vec3, w: u32, i: f32) -> Self {
        Self {
            pos: p,
            point: w != 0,
            intensity: i.clamp(0f32, 1f32),
        }
    }
}
