use crate::config::scene::*;
use crate::util::vec3::*;
use anyhow::Result;
use std::f32::consts::PI;

pub struct Dimensions {
    pub ul: Vec3,
    pub ur: Vec3,
    pub ll: Vec3,
    pub dx: Vec3,
    pub dy: Vec3,
}

#[inline]
pub fn image_setup(s: &Scene) -> Result<Dimensions> {
    let width = s.width as f32;
    let height = s.height as f32;

    let u: Vec3 = s.view.cross(s.up).norm();
    let v: Vec3 = u.cross(s.view).norm();

    // width and height of viewing window
    let aspect: f32 = width / height;
    let d: f32 = 5.0;
    let vfov_rad: f32 = s.vfov * PI / 180.0;
    let viewport_height: f32 = 2.0 * d * f32::tan(0.5 * vfov_rad);
    let viewport_width: f32 = viewport_height * aspect;

    // corners of viewing window
    let center: Vec3 = s.eye + (s.view * d);
    let ul: Vec3 = (center + (u * (-0.5 * viewport_width))) + (v * (0.5 * viewport_height));
    let ur: Vec3 = (center + (u * (0.5 * viewport_width))) + (v * (0.5 * viewport_height));
    let ll: Vec3 = (center + (u * (-0.5 * viewport_width))) + (v * (-0.5 * viewport_height));

    let dx: Vec3 = (ur - ul) * (1.0 / (width - 1.0));
    let dy: Vec3 = (ll - ul) * (1.0 / (height - 1.0));

    Ok(Dimensions {
        ul: ul,
        ur: ur,
        ll: ll,
        dx: dx,
        dy: dy,
    })
}
