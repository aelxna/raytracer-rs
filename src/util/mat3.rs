use crate::util::vec3::*;

pub type Mat3 = [[f32; 3]; 3];

#[inline]
pub fn mat3_rows(a: Vec3, b: Vec3, c: Vec3) -> Mat3 {
    [[a.x, a.y, a.z], [b.x, b.y, b.z], [c.x, c.y, c.z]]
}

#[inline]
pub fn mat3_cols(a: Vec3, b: Vec3, c: Vec3) -> Mat3 {
    [[a.x, b.x, c.x], [a.y, b.y, c.y], [a.z, b.z, c.z]]
}

pub const MAT3_ZERO: Mat3 = [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]];
