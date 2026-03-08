use crate::util::vec2::*;
use crate::util::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    diffuse: Vec3,
    specular: Vec3,
    ka: f32,
    kd: f32,
    ks: f32,
    n: f32,
    alpha: f32,
    eta: f32,
}

fn mtl_new(
    od: &Vec3,
    os: &Vec3,
    ka: f32,
    kd: f32,
    ks: f32,
    n: f32,
    alpha: f32,
    eta: f32,
) -> Material {
    Material {
        diffuse: od.clone(),
        specular: os.clone(),
        ka: ka,
        kd: kd,
        ks: ks,
        n: n,
        alpha: alpha,
        eta: eta,
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    width: u32,
    height: u32,
    img: Vec<Vec3>,
}

impl Texture {
    #[inline]
    pub fn lookup(&self, coord: &Vec2) -> Vec3 {
        let u: f32 = (coord.x % 1.0).abs();
        let v: f32 = (coord.y % 1.0).abs();

        let x: u32 = (u * ((self.width as f32) - 1.0)).round() as u32;
        let y: u32 = (v * ((self.width as f32) - 1.0)).round() as u32;

        match self.img.get((x + (y * self.width)) as usize) {
            None => self.img[0],
            Some(&color) => color,
        }
    }
}
