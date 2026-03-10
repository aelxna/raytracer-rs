use crate::io::construct::*;
use crate::util::light::*;
use crate::util::material::*;
use crate::util::shape::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use anyhow::{Context, Result};
use std::fs;
use std::rc::Rc;
use std::str;

#[derive(Debug, Clone, Default)]
pub struct Scene {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub txcoords: Vec<Vec2>,
    pub shapes: Vec<Shape>,
    pub materials: Vec<Rc<Material>>,
    pub textures: Vec<Rc<Texture>>,
    pub lights: Vec<Light>,
    pub eye: Option<Vec3>,
    pub view: Option<Vec3>,
    pub up: Option<Vec3>,
    pub bkgcolor: Option<Vec3>,
    pub eta: Option<f32>,
    pub hfov: Option<f32>,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

impl Scene {
    pub fn from_file(file_name: &str) -> Result<Self> {
        let mut scene: Self = Default::default();

        let fp = fs::read_to_string(file_name).expect("Failed to read input file");
        let lines: Vec<&str> = fp.split('\n').collect();
        for line in lines {
            let mut tokens = line.split_whitespace();

            macro_rules! parse {
                ($ty:ty) => {
                    <$ty>::construct(&mut tokens)
                        .with_context(|| format!("Failed to parse {}", stringify!($ty)))?
                };
            }

            match tokens.next() {
                None => continue,
                Some(s) => match s.as_ref() {
                    "v" => {
                        scene.vertices.push(parse!(Vec3));
                        continue;
                    }
                    "vt" => {
                        scene.txcoords.push(parse!(Vec2));
                        continue;
                    }
                    "vn" => {
                        scene.normals.push(parse!(Vec3));
                        continue;
                    }
                    "f" => continue,
                    "sphere" => {
                        let c = parse!(Vec3);
                        let r = parse!(f32);
                        let mtl = match scene.materials.last() {
                            None => continue,
                            Some(m) => m.clone(),
                        };
                        let tx = match scene.textures.last() {
                            None => None,
                            Some(t) => Some(t.clone()),
                        };

                        scene.shapes.push(Shape::Sphere(Sphere::new(c, r, mtl, tx)));
                        continue;
                    }
                    "light" => continue,
                    "mtlcolor" => continue,
                    "texture" => continue,
                    "eye" => continue,
                    "viewdir" => continue,
                    "updir" => continue,
                    "bkgcolor" => continue,
                    "hfov" => continue,
                    "imsize" => continue,
                    _ => continue,
                },
            }
        }

        Ok(scene)
    }
}
