use crate::config::construct::*;
use crate::entities::light::*;
use crate::entities::shape::*;
use crate::entities::sphere::*;
use crate::entities::triangle::*;
use crate::util::mat4::*;
use crate::util::material::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use crate::util::vec4::*;
use anyhow::{Context, Result, bail};
use image::RgbImage;
use std::fs;
use std::str;
use std::sync::Arc;

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

        let vertices: [Arc<Vec3>; 3] = vi.map(|i| unwrap!(scene.vertices, i));

        let mtl = match scene.materials.last() {
            None => bail!("Defined triangle without first defining material"),
            Some(r) => r.clone(),
        };

        Ok(Triangle::new(TriNormMode::Flat, vertices, mtl, None, None))
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

        let vertices: [Arc<Vec3>; 3] = [
            unwrap!(scene.vertices, vi1),
            unwrap!(scene.vertices, vi2),
            unwrap!(scene.vertices, vi3),
        ];

        let vt: [Arc<Vec2>; 3] = [
            unwrap!(scene.texcoords, vt1),
            unwrap!(scene.texcoords, vt2),
            unwrap!(scene.texcoords, vt3),
        ];

        let mtl = match scene.materials.last() {
            None => bail!("Defined triangle without first defining material"),
            Some(r) => r.clone(),
        };

        let tx = match scene.textures.last() {
            None => None,
            Some(r) => Some(r.clone()),
        };

        let mode = match scene.normalmaps.last() {
            None => TriNormMode::Flat,
            Some(r) => TriNormMode::Map(r.clone()),
        };

        Ok(Triangle::new(mode, vertices, mtl, tx, Some(vt)))
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

        let vertices: [Arc<Vec3>; 3] = [
            unwrap!(scene.vertices, vi1),
            unwrap!(scene.vertices, vi2),
            unwrap!(scene.vertices, vi3),
        ];

        let vn: [Arc<Vec3>; 3] = [
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
                None => None,
                Some(r) => Some(r.clone()),
            };

            let vti: [usize; 3] = [
                v1[1].parse::<usize>()? - 1,
                v2[1].parse::<usize>()? - 1,
                v3[1].parse::<usize>()? - 1,
            ];

            let vt: [Arc<Vec2>; 3] = vti.map(|i| unwrap!(scene.texcoords, i));

            Ok(Triangle::new(
                TriNormMode::Smooth(Vec::from(&vn)),
                vertices,
                mtl,
                tx,
                Some(vt),
            ))
        } else {
            Ok(Triangle::new(
                TriNormMode::Smooth(Vec::from(&vn)),
                vertices,
                mtl,
                None,
                None,
            ))
        }
    } else {
        bail!("Incorrect format for triangle")
    }
}

#[inline]
fn make_transformation_matrices(eye: Vec3, view: Vec3, up: Vec3) -> (Mat4, Mat4) {
    let r = view.cross(up).norm();
    let u = r.cross(view).norm();

    let a = Vec4::from_vec3(r, r.dot(-eye));
    let b = Vec4::from_vec3(u, u.dot(-eye));
    let c = Vec4::from_vec3(-view, view.dot(eye));
    let d = Vec4::new(0.0, 0.0, 0.0, 1.0);

    (mat4_rows(a, b, c, d), mat4_cols(a, b, c, d))
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub vertices: Vec<Arc<Vec3>>,
    pub normals: Vec<Arc<Vec3>>,
    pub texcoords: Vec<Arc<Vec2>>,
    pub shapes: Vec<Shape>,
    pub materials: Vec<Arc<Material>>,
    pub textures: Vec<Arc<RgbImage>>,
    pub normalmaps: Vec<Arc<RgbImage>>,
    pub lights: Vec<Light>,
    pub eye: Vec3,
    pub view: Vec3,
    pub up: Vec3,
    pub bkgcolor: Vec3,
    pub eta: f32,
    pub vfov: f32,
    pub width: u32,
    pub height: u32,
    pub world_to_camera: Mat4,
    pub camera_to_world: Mat4,
}

impl Scene {
    pub fn from(file_name: &str) -> Result<Self> {
        let mut scene: Self = Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            texcoords: Vec::new(),
            shapes: Vec::new(),
            materials: Vec::new(),
            textures: Vec::new(),
            normalmaps: Vec::new(),
            lights: Vec::new(),
            eye: Vec3::ZERO,
            view: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            bkgcolor: Vec3::ZERO,
            eta: 1.0,
            vfov: 50.0,
            width: 1,
            height: 1,
            world_to_camera: MAT4_ZERO,
            camera_to_world: MAT4_ZERO,
        };

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
                        scene.vertices.push(Arc::new(parse!(Vec3)));
                        continue;
                    }
                    "vt" => {
                        scene.texcoords.push(Arc::new(parse!(Vec2)));
                        continue;
                    }
                    "vn" => {
                        scene.normals.push(Arc::new(parse!(Vec3)));
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
                            .push(Arc::new(Material::new(od, os, ka, kd, ks, n, alpha, eta)));
                        continue;
                    }
                    "texture" => match tokens.next() {
                        None => continue,
                        Some(f) => {
                            let resolved = std::path::Path::new(file_name)
                                .parent()
                                .unwrap_or(std::path::Path::new("."))
                                .join(f);

                            let img = image::open(&resolved)
                                .with_context(|| format!("Failed to open texture {}", f))?
                                .into_rgb8();

                            scene.textures.push(Arc::new(img));
                            continue;
                        }
                    },
                    "norm" => match tokens.next() {
                        None => continue,
                        Some(f) => {
                            let resolved = std::path::Path::new(file_name)
                                .parent()
                                .unwrap_or(std::path::Path::new("."))
                                .join(f);

                            let img = image::open(&resolved)
                                .with_context(|| format!("Failed to open texture {}", f))?
                                .into_rgb8();

                            scene.normalmaps.push(Arc::new(img));
                            continue;
                        }
                    },
                    "eye" => {
                        scene.eye = parse!(Vec3);
                        continue;
                    }
                    "viewdir" => {
                        scene.view = parse!(Vec3).norm();
                        continue;
                    }
                    "updir" => {
                        scene.up = parse!(Vec3).norm();
                        continue;
                    }
                    "bkgcolor" => {
                        scene.bkgcolor = parse!(Vec3).clamp(0.0, 1.0);
                        scene.eta = parse!(f32);
                        continue;
                    }
                    "vfov" => {
                        scene.vfov = parse!(f32);
                        continue;
                    }
                    "imsize" => {
                        scene.width = parse!(u32);
                        scene.height = parse!(u32);
                    }
                    _ => continue,
                },
            }
        }

        // (scene.world_to_camera, scene.camera_to_world) =
        //     make_transformation_matrices(scene.eye, scene.view, scene.up);

        // TODO: for each shape:
        // sphere -> convert center to camera space, place into boxes based on +- radius in x and y
        // triangle ->

        Ok(scene)
    }
}
