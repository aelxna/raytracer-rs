use crate::config::scene::*;
use crate::util::vec3::*;
use anyhow::{Result, anyhow};
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
    let viewdir: Vec3 = s.view.ok_or(anyhow!("Failed to retrieve viewdir"))?;
    let updir: Vec3 = s.up.ok_or(anyhow!("Failed to retrieve updir"))?;
    let eye: Vec3 = s.eye.ok_or(anyhow!("Failed to retrieve eye"))?;
    let width: f32 = s.width.ok_or(anyhow!("Failed to retrieve width"))? as f32;
    let height: f32 = s.height.ok_or(anyhow!("Failed to retrieve height"))? as f32;
    let vfov: f32 = s.vfov.ok_or(anyhow!("Failed to retrieve fov"))?;

    let u: Vec3 = viewdir.cross(updir).norm();
    let v: Vec3 = u.cross(viewdir).norm();

    // width and height of viewing window
    let aspect: f32 = width / height;
    let d: f32 = 5.0;
    let vfov_rad: f32 = vfov * PI / 180.0;
    let viewport_height: f32 = 2.0 * d * f32::tan(0.5 * vfov_rad);
    let viewport_width: f32 = viewport_height * aspect;

    // corners of viewing window
    let center: Vec3 = eye + (viewdir * d);
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
