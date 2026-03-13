use crate::io::construct::*;
use crate::util::light::*;
use crate::util::material::*;
use crate::util::shape::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use anyhow::{Context, Result, bail};
use std::fs;
use std::rc::Rc;
use std::str;

fn parse_triangle(scene: &Scene, it: &mut str::SplitWhitespace<'_>) -> Result<Triangle> {
    let v1: Vec<&str> = match it.next() {
        None => bail!("Failed to parse triangle"),
        Some(tok) => tok.split('/').collect(),
    };

    macro_rules! parse {
        ($e:expr) => {
            match $e.next() {
                None => bail!("Incorrect format for triangle"),
                Some(u) => u.parse::<usize>()?,
            }
        };
    }

    macro_rules! unwrap {
        ($loc:expr, $i:expr) => {
            $loc.get($i).unwrap().clone()
        };
    }

    if v1.len() == 1 {
        // just vertices

        let vi: [usize; 3] = [v1[0].parse::<usize>()? - 1, parse!(it) - 1, parse!(it) - 1];

        let vertices: [Vec3; 3] = vi.map(|i| unwrap!(scene.vertices, i));

        let mtl = match scene.materials.last() {
            None => bail!("Defined triangle without first defining material"),
            Some(r) => r.clone(),
        };

        Ok(Triangle::new(vertices, None, mtl, None, None))
    } else if v1.len() == 2 {
        // + txcoords

        // get the other fields
        let v2: Vec<&str> = match it.next() {
            None => bail!("Failed to parse triangle"),
            Some(tok) => tok.split('/').collect(),
        };
        let v3: Vec<&str> = match it.next() {
            None => bail!("Failed to parse triangle"),
            Some(tok) => tok.split('/').collect(),
        };

        // get the first set of coords
        let vi1 = v1[0].parse::<usize>()? - 1;
        let vt1 = v1[1].parse::<usize>()? - 1;

        // get the second set of coords
        let vi2 = v2[0].parse::<usize>()? - 1;
        let vt2 = v2[1].parse::<usize>()? - 1;

        // get the third set of coords
        let vi3 = v3[0].parse::<usize>()? - 1;
        let vt3 = v3[1].parse::<usize>()? - 1;

        let vertices: [Vec3; 3] = [
            unwrap!(scene.vertices, vi1),
            unwrap!(scene.vertices, vi2),
            unwrap!(scene.vertices, vi3),
        ];

        let vt: [Vec2; 3] = [
            unwrap!(scene.txcoords, vt1),
            unwrap!(scene.txcoords, vt2),
            unwrap!(scene.txcoords, vt3),
        ];

        let mtl = match scene.materials.last() {
            None => bail!("Defined triangle without first defining material"),
            Some(r) => r.clone(),
        };

        let tx = match scene.textures.last() {
            None => {
                bail!("Defined triangle with texture coordinates without first defining texture")
            }
            Some(r) => r.clone(),
        };

        Ok(Triangle::new(vertices, None, mtl, Some(tx), Some(vt)))
    } else if v1.len() == 3 {
        // + txcoords and normals

        // get the other fields
        let v2: Vec<&str> = match it.next() {
            None => bail!("Failed to parse triangle"),
            Some(tok) => tok.split('/').collect(),
        };

        let v3: Vec<&str> = match it.next() {
            None => bail!("Failed to parse triangle"),
            Some(tok) => tok.split('/').collect(),
        };

        // check if there are texture coordinates
        let use_tx = match v1[1].as_ref() {
            "" => false,
            _ => true,
        };

        // get the first set of coords
        let vi1 = v1[0].parse::<usize>()? - 1;
        let vn1 = v1[2].parse::<usize>()? - 1;

        // get the second set of coords
        let vi2 = v2[0].parse::<usize>()? - 1;
        let vn2 = v2[2].parse::<usize>()? - 1;

        // get the third set of coords
        let vi3 = v3[0].parse::<usize>()? - 1;
        let vn3 = v3[2].parse::<usize>()? - 1;

        let vertices: [Vec3; 3] = [
            unwrap!(scene.vertices, vi1),
            unwrap!(scene.vertices, vi2),
            unwrap!(scene.vertices, vi3),
        ];

        let vn: [Vec3; 3] = [
            unwrap!(scene.normals, vn1),
            unwrap!(scene.normals, vn2),
            unwrap!(scene.normals, vn3),
        ];

        let mtl = match scene.materials.last() {
            None => bail!("Defined triangle without first defining material"),
            Some(r) => r.clone(),
        };

        if use_tx {
            let tx = match scene.textures.last() {
                None => {
                    bail!(
                        "Defined triangle with texture coordinates without first defining texture"
                    )
                }
                Some(r) => r.clone(),
            };

            let vti: [usize; 3] = [
                v1[1].parse::<usize>()? - 1,
                v2[1].parse::<usize>()? - 1,
                v3[1].parse::<usize>()? - 1,
            ];

            let vt: [Vec2; 3] = vti.map(|i| unwrap!(scene.txcoords, i));

            Ok(Triangle::new(vertices, Some(vn), mtl, Some(tx), Some(vt)))
        } else {
            Ok(Triangle::new(vertices, Some(vn), mtl, None, None))
        }
    } else {
        bail!("Incorrect format for triangle")
    }
}

fn validate_fields(scene: &Scene) -> bool {
    scene.materials.len() > 0
        && scene.eye != None
        && scene.view != None
        && scene.up != None
        && scene.bkgcolor != None
        && scene.eta != None
        && scene.hfov != None
        && scene.width != None
        && scene.height != None
}

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
    pub fn from(file_name: &str) -> Result<Self> {
        let mut scene: Self = Default::default();

        let fp = fs::read_to_string(file_name).expect("Failed to read input file");
        let lines: Vec<&str> = fp.split('\n').collect();
        for line in lines {
            let mut tokens = line.split_whitespace();

            macro_rules! parse {
                ($ty:ty) => {
                    <$ty>::construct(&mut tokens).with_context(|| {
                        format!(
                            "Failed to parse {}, {}:{}",
                            stringify!($ty),
                            file!(),
                            line!()
                        )
                    })?
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
                    "f" => {
                        scene
                            .shapes
                            .push(Shape::Triangle(parse_triangle(&scene, &mut tokens)?));
                        continue;
                    }
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
                    "light" => {
                        let p = parse!(Vec3);
                        let w = parse!(u32);
                        let i = parse!(f32);

                        scene.lights.push(Light::new(p, w, i));
                        continue;
                    }
                    "mtlcolor" => {
                        let od = parse!(Vec3).clamp(0.0, 1.0);
                        let os = parse!(Vec3).clamp(0.0, 1.0);
                        let ka = parse!(f32);
                        let kd = parse!(f32);
                        let ks = parse!(f32);
                        let n = parse!(f32);
                        let alpha = parse!(f32);
                        let eta = parse!(f32);

                        scene
                            .materials
                            .push(Rc::new(Material::new(od, os, ka, kd, ks, n, alpha, eta)));
                        continue;
                    }
                    "texture" => match tokens.next() {
                        None => continue,
                        Some(f) => {
                            scene.textures.push(Rc::new(Texture::from(f)?));
                            continue;
                        }
                    },
                    "eye" => {
                        scene.eye = Some(parse!(Vec3));
                        continue;
                    }
                    "viewdir" => {
                        scene.view = Some(parse!(Vec3).norm());
                        continue;
                    }
                    "updir" => {
                        scene.up = Some(parse!(Vec3).norm());
                        continue;
                    }
                    "bkgcolor" => {
                        scene.bkgcolor = Some(parse!(Vec3).clamp(0.0, 1.0));
                        scene.eta = Some(parse!(f32));
                        continue;
                    }
                    "hfov" => {
                        scene.hfov = Some(parse!(f32));
                        continue;
                    }
                    "imsize" => {
                        scene.width = Some(parse!(usize));
                        scene.height = Some(parse!(usize));
                    }
                    _ => continue,
                },
            }
        }

        if validate_fields(&scene) {
            Ok(scene)
        } else {
            bail!("wehh")
        }
    }
}
