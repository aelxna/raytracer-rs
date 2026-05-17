use crate::io::construct::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use anyhow::{Context, Result, bail};
use image::RgbImage;
use std::fs;
use std::rc::Rc;
use std::str;

#[derive(Debug, Clone)]
pub struct Material {
    diffuse: Vec3,
    specular: Vec3,
    ka: f32,
    kd: f32,
    ks: f32,
    exp: f32,
    alpha: f32,
    eta: f32,
}

impl Material {
    #[inline]
    pub fn new(
        od: Vec3,
        os: Vec3,
        ka: f32,
        kd: f32,
        ks: f32,
        n: f32,
        alpha: f32,
        eta: f32,
    ) -> Self {
        Self {
            diffuse: od.clamp(0.0, 1.0),
            specular: os.clamp(0.0, 1.0),
            ka: ka,
            kd: kd,
            ks: ks,
            exp: n,
            alpha: alpha.clamp(0.0, 1.0),
            eta: eta,
        }
    }
}

#[inline]
pub fn texture_lookup(img: Rc<RgbImage>, coord: Vec2) -> Result<Vec3> {
    //
    let u: u32 = coord.x.floor().abs() as u32;
    let v: u32 = coord.y.floor().abs() as u32;

    let pixel = img
        .get_pixel_checked(u, v)
        .with_context(|| "Failed to retrieve texture coordinate")?;

    Ok(Vec3::from_rgb(pixel.0))
}
