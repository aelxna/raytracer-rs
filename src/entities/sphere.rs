use crate::util::material::*;
use crate::util::vec3::*;
use image::RgbImage;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub mode: SphNormMode,
    pub center: Vec3,
    pub radius: f32,
    pub mtl: Arc<Material>,
    pub texture: Option<Arc<RgbImage>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SphNormMode {
    Flat,
    Map(Arc<RgbImage>),
}

impl Sphere {
    #[inline]
    pub fn new(
        mode: SphNormMode,
        c: Vec3,
        r: f32,
        mtl: Arc<Material>,
        tx: Option<Arc<RgbImage>>,
    ) -> Self {
        Self {
            mode: mode,
            center: c,
            radius: r,
            mtl: mtl,
            texture: tx,
        }
    }
}
