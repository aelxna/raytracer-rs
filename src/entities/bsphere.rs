use crate::util::vec3::*;

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
    pub shape_indices: Vec<usize>,
}

impl BoundingSphere {
    #[inline]
    pub fn new(c: Vec3, r: f32) -> Self {
        Self {
            center: c,
            radius: r,
            shape_indices: Vec::new(),
        }
    }
}
