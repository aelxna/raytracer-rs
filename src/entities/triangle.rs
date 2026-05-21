use crate::util::material::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use image::RgbImage;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub vertices: [Arc<Vec3>; 3],
    pub normals: Option<[Arc<Vec3>; 3]>,
    pub e1: Vec3,
    pub e2: Vec3,
    pub snorm: Vec3,
    pub d: f32,
    pub mtl: Arc<Material>,
    pub texture: Option<Arc<RgbImage>>,
    pub texcoords: Option<[Arc<Vec2>; 3]>,
}

impl Triangle {
    #[inline]
    pub fn new(
        v: [Arc<Vec3>; 3],
        n: Option<[Arc<Vec3>; 3]>,
        mtl: Arc<Material>,
        tx: Option<Arc<RgbImage>>,
        tc: Option<[Arc<Vec2>; 3]>,
    ) -> Self {
        let e1 = *v[1] - *v[0];
        let e2 = *v[2] - *v[0];
        let snorm = e1.cross(e2).norm();
        let d = -snorm.dot(*v[1]);

        Self {
            vertices: v,
            normals: n,
            e1: e1,
            e2: e2,
            snorm: snorm,
            d: d,
            mtl: mtl,
            texture: tx,
            texcoords: tc,
        }
    }
}
