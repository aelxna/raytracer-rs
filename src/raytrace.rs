use crate::io::scene::Scene;
use crate::util::light::*;
use crate::util::material::*;
use crate::util::shape::*;
use crate::util::vec3::*;
use anyhow::{Context, Result, anyhow, bail};
use std::f32::consts::PI;

const ERR_BOUND: f32 = 0.001;
const BOUNCES: usize = 10;

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
    let viewport_width: f32 = height * aspect;

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

struct Trace<'a> {
    shape: Option<&'a Shape>,
    b: f32,
    g: f32,
    t: f32,
    shadow: f32,
}

#[inline]
fn trace_ray<'a>(r: Ray3, shapes: &'a Vec<Shape>, skip: Option<&'a Shape>, l: f32) -> Trace<'a> {
    let mut ret: Trace = Trace {
        shape: None,
        b: -1.0,
        g: -1.0,
        t: -1.0,
        shadow: 1.0,
    };

    let mut si: Option<&Shape> = None;

    for s in shapes {
        if match skip {
            None => false,
            Some(sk) => s == sk,
        } {
            continue;
        }

        match s {
            Shape::Sphere(sp) => {
                let b: f32 = 2.0 * r.dir.dot(r.origin - sp.center);
                let constant: f32 = (r.origin - sp.center).sq_mag() - (sp.radius * sp.radius);

                let mut t1 = -1.0;
                let mut t2 = -1.0;

                // do the quadratic formula to find intersection points
                let discrim: f32 = (b * b) - (4.0 * constant);
                if discrim >= 0.0 {
                    t1 = -b + f32::sqrt(discrim) / 2.0;
                    t2 = -b - f32::sqrt(discrim) / 2.0;
                }

                if (t1 > ERR_BOUND) && (t1 < l) {
                    ret.shadow *= 1.0 - sp.mtl.alpha;
                }
                if (t2 > ERR_BOUND) && (t2 < l) {
                    ret.shadow *= 1.0 - sp.mtl.alpha;
                }

                // pick t that is closest to origin and still positive
                if (t1 > ERR_BOUND) && (t1 <= t2 || t2 <= ERR_BOUND) && (t1 < ret.t || ret.t < 0.0)
                {
                    ret.t = t1;
                    si = Some(s);
                } else if t2 > ERR_BOUND && (t2 < ret.t || ret.t < 0.0) {
                    ret.t = t2;
                    si = Some(s);
                }
            }
            Shape::Triangle(tr) => {
                if tr.snorm.dot(r.dir) == 0.0 {
                    continue;
                }

                // find collision with plane
                let t: f32 = -(tr.snorm.dot(r.origin) + tr.d) / tr.snorm.dot(r.dir);
                let pt: Vec3 = r.origin + (r.dir * t);
                let ep: Vec3 = pt - tr.vertices[0];
                let det: f32 =
                    (tr.e1.sq_mag() * tr.e2.sq_mag()) - (tr.e1.dot(tr.e2) * tr.e1.dot(ep));
                if det == 0.0 {
                    continue;
                }

                // get barycentric coordinates
                let beta: f32 =
                    ((tr.e2.sq_mag() * tr.e1.dot(ep)) - (tr.e1.dot(tr.e2) * tr.e2.dot(ep))) / det;
                let gamma: f32 =
                    ((tr.e1.sq_mag() * tr.e2.dot(ep)) - (tr.e1.dot(tr.e2) * tr.e1.dot(ep))) / det;
                if beta < 0.0 || gamma < 0.0 {
                    continue;
                } else if beta + gamma > 1.0 {
                    continue;
                } else {
                    if t > ERR_BOUND && t < l {
                        ret.shadow *= 1.0 - tr.mtl.alpha;
                    }
                    if t > ERR_BOUND && (t < ret.t || ret.t < 0.0) {
                        ret.b = beta;
                        ret.g = gamma;
                        ret.t = t;
                        si = Some(s);
                    }
                }
            }
        }
    }
    ret.shape = si;
    ret
}

#[inline]
fn diffuse_normal(diffuse: &mut Vec3, normal: &mut Vec3, illum: &mut Vec3, tr: &Trace) -> () {}

#[inline]
fn setup_eta(normal: &mut Vec3, vi: &Vec3, shape: &Shape, stack: &mut Vec<f32>) -> (f32, f32) {
    (0.0, 0.0)
}

#[inline]
fn apply_lighting(
    lights: &Vec<Light>,
    p: &Vec3,
    normal: &Vec3,
    vi: &Vec3,
    diffuse: &Vec3,
    shape: &Shape,
    shapes: &Vec<Shape>,
    illum: &mut Vec3,
) -> () {
}

#[inline]
fn reflections_transparency(
    p: &Vec3,
    normal: &Vec3,
    vi: &Vec3,
    shape: &Shape,
    eta_i: f32,
    eta_t: f32,
    scene: &Scene,
    acc: usize,
    skip: Option<&Shape>,
    stack: &mut Vec<f32>,
    illum: &mut f32,
) -> () {
}

#[inline]
pub fn shade_ray(
    r: Ray3,
    scene: &Scene,
    acc: usize,
    skip: Option<&Shape>,
    stack: &mut Vec<f32>,
) -> Vec3 {
    let bkgcolor: Vec3 = scene.bkgcolor.unwrap_or(Vec3::ZERO);

    // cap on how many times reflection can take place
    if acc > BOUNCES {
        return bkgcolor;
    }

    let tr: Trace = trace_ray(r, &scene.shapes, skip, -1.0);
    let mut illum: Vec3 = Vec3::ZERO;
    let mut diffuse: Vec3 = Vec3::ZERO;
    let mut normal: Vec3 = Vec3::ZERO;

    // no intersection
    if tr.shape == None {
        return bkgcolor;
    }
    let shape: &Shape = tr.shape.unwrap(); // must not be None

    diffuse_normal(&mut diffuse, &mut normal, &mut illum, &tr);

    let p: Vec3 = r.origin + (r.dir * tr.t);
    let vi: Vec3 = (-r.dir).norm();

    let (eta_i, eta_t) = setup_eta(&mut normal, &vi, shape, stack);

    apply_lighting(
        &scene.lights,
        &p,
        &normal,
        &vi,
        &diffuse,
        shape,
        &scene.shapes,
        &mut illum,
    );

    illum.clamp(0.0, 1.0)
}
