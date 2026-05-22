use crate::util::vec3::*;
use crate::util::vec4::*;

pub type Mat4 = [[f32; 4]; 4];

#[inline]
pub fn mat4_rows(a: Vec4, b: Vec4, c: Vec4, d: Vec4) -> Mat4 {
    [
        [a.x, a.y, a.z, a.w],
        [b.x, b.y, b.z, b.w],
        [c.x, c.y, c.z, c.w],
        [d.x, d.y, d.z, d.w],
    ]
}

#[inline]
pub fn mat4_cols(a: Vec4, b: Vec4, c: Vec4, d: Vec4) -> Mat4 {
    [
        [a.x, b.x, c.x, d.x],
        [a.y, b.y, c.y, d.y],
        [a.z, b.z, c.z, d.z],
        [a.w, b.w, c.w, d.w],
    ]
}

#[inline]
pub fn transform(m: Mat4, x: Vec4) -> Vec3 {
    Vec3::new(
        Vec4::from(m[0]).dot(x),
        Vec4::from(m[1]).dot(x),
        Vec4::from(m[2]).dot(x),
    )
}

pub const MAT4_ZERO: Mat4 = [
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0],
];
