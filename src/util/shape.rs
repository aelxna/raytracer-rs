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

impl Sphere {
    #[inline]
    pub fn new(c: Vec3, r: f32, mtl: &'static Material, tx: Option<&'static Texture>) -> Self {
        Self {
            center: c,
            radius: r,
            mtl: mtl,
            texture: tx,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [Vec3; 3],
    pub normals: [Vec3; 3],
    pub e1: Vec3,
    pub e2: Vec3,
    pub snorm: Vec3,
    pub d: f32,
    pub mtl: &'static Material,
    pub texture: Option<&'static Texture>,
    pub texcoords: Option<[Vec2; 3]>,
}

impl Triangle {
    pub fn new(
        v: [Vec3; 3],
        n: [Vec3; 3],
        mtl: &'static Material,
        tx: Option<&'static Texture>,
        tc: Option<[Vec2; 3]>,
    ) -> Self {
        let e1 = v[1] - v[0];
        let e2 = v[2] - v[0];
        let snorm = e1.cross(e2).norm();
        let d = -snorm.dot(v[1]);

        Self {
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
}

enum Shape {
    Sphere(Sphere),
    Triangle(Triangle),
}
