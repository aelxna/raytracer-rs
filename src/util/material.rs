use crate::io::construct::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use anyhow::{Context, Result, bail};
use image::RgbImage;
use std::fs;
use std::rc::Rc;
use std::str;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub diffuse: Vec3,
    pub specular: Vec3,
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub exp: f32,
    pub alpha: f32,
    pub eta: f32,
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
fn nearest_neighbor(img: &RgbImage, coord: Vec2) -> Result<Vec3> {
    //
    let u: u32 = coord.x.floor().abs() as u32;
    let v: u32 = coord.y.floor().abs() as u32;

    let pixel = img
        .get_pixel_checked(u, v)
        .with_context(|| "Failed to retrieve texture coordinate")?;

    Ok(Vec3::from_rgb(pixel.0))
}

#[inline]
fn bilinear_interpolate(img: &RgbImage, coord: Vec2) -> Result<Vec3> {
    let unit: Vec2 = Vec2::new(coord.x.floor(), coord.y.floor());

    let tl = nearest_neighbor(img, unit)?;
    let tr = nearest_neighbor(img, Vec2::new(unit.x + 1.0, unit.y))?;
    let bl = nearest_neighbor(img, Vec2::new(unit.x, unit.y + 1.0))?;
    let br = nearest_neighbor(img, Vec2::new(unit.x + 1.0, unit.y + 1.0))?;

    let top = tl.lerp(tr, coord.x - unit.x);
    let btm = bl.lerp(br, coord.x - unit.x);

    Ok(top.lerp(btm, coord.y - unit.y))
}

#[inline]
pub fn texture_lookup(img: &RgbImage, coord: Vec2) -> Result<Vec3> {
    bilinear_interpolate(img, coord)
}
