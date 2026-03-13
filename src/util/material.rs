use crate::io::construct::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use anyhow::{Context, Result, bail};
use std::fs;
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

#[derive(Debug, Clone)]
pub struct Texture {
    width: usize,
    height: usize,
    img: Vec<Vec3>,
}

impl Texture {
    #[inline]
    pub fn new(w: usize, h: usize, img: Vec<Vec3>) -> Self {
        Self {
            width: w,
            height: h,
            img: img,
        }
    }

    pub fn from(file_name: &str) -> Result<Self> {
        let fp = fs::read_to_string(file_name)?;
        let lines: Vec<&str> = fp.split('\n').collect();

        let mut meta = match lines.get(0) {
            None => bail!("Texture file is empty"),
            Some(ln) => ln.split_whitespace(),
        };

        meta.next();
        let width = <usize>::construct(&mut meta)?;
        let height = <usize>::construct(&mut meta)?;

        let mut it = match lines.get(1) {
            None => bail!("Texture file formatted incorrectly"),
            Some(ln) => ln.split_whitespace(),
        };

        macro_rules! parse_color {
            () => {
                Vec3::new(
                    (<usize>::construct(&mut it)
                        .with_context(|| format!("Failed to parse color"))? as f32
                        / 255.0)
                        .clamp(0.0, 1.0),
                    (<usize>::construct(&mut it)
                        .with_context(|| format!("Failed to parse color"))? as f32
                        / 255.0)
                        .clamp(0.0, 1.0),
                    (<usize>::construct(&mut it)
                        .with_context(|| format!("Failed to parse color"))? as f32
                        / 255.0)
                        .clamp(0.0, 1.0),
                )
            };
        }

        let mut img: Vec<Vec3> = Vec::new();
        dbg!(width * height);

        for _ in 0..(width * height) {
            img.push(parse_color!());
        }

        Ok(Texture::new(width, height, img))
    }

    #[inline]
    pub fn lookup(&self, coord: Vec2) -> Vec3 {
        let u: f32 = (coord.x % 1.0).abs();
        let v: f32 = (coord.y % 1.0).abs();

        let x: usize = (u * ((self.width as f32) - 1.0)).round() as usize;
        let y: usize = (v * ((self.height as f32) - 1.0)).round() as usize;

        match self.img.get(x + (y * self.width)) {
            None => self.img[0],
            Some(&color) => color,
        }
    }
}
