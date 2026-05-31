use crate::util::mat3::*;
use crate::util::material::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use image::RgbImage;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub mode: TriNormMode,
    pub vertices: [Arc<Vec3>; 3],
    pub e1: Vec3,
    pub e2: Vec3,
    pub snorm: Vec3,
    pub d: f32,
    pub tbn: Mat3,
    pub mtl: Arc<Material>,
    pub texture: Option<Arc<RgbImage>>,
    pub txcoords: Option<[Arc<Vec2>; 3]>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TriNormMode {
    Flat,
    Smooth(Vec<Arc<Vec3>>),
    Map(Arc<RgbImage>),
}

impl Triangle {
    #[inline]
    pub fn new(
        mode: TriNormMode,
        v: [Arc<Vec3>; 3],
        mtl: Arc<Material>,
        tx: Option<Arc<RgbImage>>,
        tc: Option<[Arc<Vec2>; 3]>,
    ) -> Self {
        let e1 = *v[1] - *v[0];
        let e2 = *v[2] - *v[0];
        let snorm = e1.cross(e2).norm();
        let d = -snorm.dot(*v[1]);

        let tbn: Mat3 = match (&tc, &mode) {
            (Some(c), TriNormMode::Map(_)) => {
                ();
                let deltauv1 = *c[1] - *c[0];
                let deltauv2 = *c[2] - *c[0];

                let f: f32 = 1.0 / (deltauv1.x * deltauv1.y - deltauv2.x * deltauv2.y);

                let t: Vec3 = Vec3::new(
                    f * (deltauv2.y * e1.x - deltauv1.y * e2.x),
                    f * (deltauv2.y * e1.y - deltauv1.y * e2.y),
                    f * (deltauv2.y * e1.z - deltauv1.y * e2.z),
                )
                .norm();

                let b: Vec3 = snorm.cross(t);

                mat3_rows(t, b, snorm)
            }
            (_, _) => MAT3_ZERO,
        };

        Self {
            mode: mode,
            vertices: v,
            e1: e1,
            e2: e2,
            snorm: snorm,
            d: d,
            tbn: tbn,
            mtl: mtl,
            texture: tx,
            txcoords: tc,
        }
    }
}
