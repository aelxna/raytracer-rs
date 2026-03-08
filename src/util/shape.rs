use crate::util::material::*;
use crate::util::vec2::*;
use crate::util::vec3::*;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mtl: &'static Material,
    pub texture: Option<&'static Texture>,
}

fn sphere_new(c: Vec3, r: f32, mtl: &'static Material, tx: Option<&'static Texture>) -> Sphere {
    Sphere {
        center: c,
        radius: r,
        mtl: mtl,
        texture: tx,
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [&'static Vec3; 3],
    pub normals: [&'static Vec3; 3],
    pub e1: Vec3,
    pub e2: Vec3,
    pub snorm: Vec3,
    pub d: f32,
    pub mtl: &'static Material,
    pub texture: Option<&'static Texture>,
    pub texcoords: Option<[&'static Vec2; 3]>,
}

fn triangle_new(
    v: [&'static Vec3; 3],
    n: [&'static Vec3; 3],
    mtl: &'static Material,
    tx: Option<&'static Texture>,
    tc: Option<[&'static Vec2; 3]>,
) -> Triangle {
    let e1 = v[1].clone() - v[0].clone();
    let e2 = v[2].clone() - v[0].clone();
    let snorm = e1.cross(&e2).norm();
    let d = -snorm.dot(v[1]);

    Triangle {
        vertices: v,
        normals: n,
        e1: e1,
        e2: e2,
        snorm: snorm,
        d: d,
        mtl: mtl,
        texture: tx,
        texcoords: tc,
    }
}

enum Shape {
    Sphere(Sphere),
    Triangle(Triangle),
}
