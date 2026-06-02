use crate::config::scene::Scene;
use crate::entities::bsphere::*;
use crate::entities::light::*;
use crate::entities::shape::sphere::SphNormMode;
use crate::entities::shape::triangle::TriNormMode;
use crate::entities::shape::*;
use crate::render::trace::*;
use crate::util::mat3::*;
use crate::util::material::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use std::f32::consts::PI;
use std::sync::Arc;

const BOUNCES: usize = 10;

#[inline]
fn diffuse_normal(
    diffuse: &mut Vec3,
    normal: &mut Vec3,
    illum: &mut Vec3,
    p: Vec3,
    tr: &Trace,
) -> () {
    match &tr.shape {
        Some(sh) => match &**sh {
            Shape::Sphere(s) => {
                let mut coord = Vec2::ZERO;
                match s.texture.clone() {
                    None => {
                        *diffuse = s.mtl.diffuse;
                    }
                    Some(tx) => {
                        // replace diffuse with texture lookup
                        let sphere_norm: Vec3 = (p - s.center) * (1.0 / s.radius);
                        let phi: f32 = f32::acos(sphere_norm.y);
                        let mut theta: f32 = f32::atan2(sphere_norm.z, sphere_norm.x);
                        if theta < 0.0 {
                            theta += 2.0 * PI;
                        }
                        coord = Vec2::new(theta / (2.0 * PI), phi / PI);

                        *diffuse = texture_lookup(&tx, coord).unwrap_or(Vec3::ZERO);
                    }
                }

                *illum = *diffuse * s.mtl.ka;
                *normal = match &s.mode {
                    SphNormMode::Flat => (p - s.center).norm(),
                    SphNormMode::Map(m) => {
                        let n = (p - s.center).norm();
                        let t = Vec3::new(-n.z, 0.0, n.x).norm();
                        let b = n.cross(t);
                        let tbn = mat3_rows(t, b, n);

                        texture_lookup(&m, coord)
                            .unwrap_or(Vec3::new(0.0, 1.0, 0.0))
                            .norm()
                            .transform(tbn)
                    }
                };
            }
            Shape::Triangle(t) => {
                let alpha: f32 = 1.0 - (tr.b + tr.g);
                let mut coord = Vec2::ZERO;
                match t.texture.clone() {
                    None => {
                        *diffuse = t.mtl.diffuse;
                    }
                    Some(tx) => {
                        // replace diffuse with texture lookup
                        let txcoords = t.txcoords.clone().unwrap_or([
                            Arc::new(Vec2::ZERO),
                            Arc::new(Vec2::ZERO),
                            Arc::new(Vec2::ZERO),
                        ]);
                        coord = Vec2::new(
                            (alpha * txcoords[0].x)
                                + (tr.b * txcoords[1].x)
                                + (tr.g * txcoords[2].x),
                            (alpha * txcoords[0].y)
                                + (tr.b * txcoords[1].y)
                                + (tr.g * txcoords[2].y),
                        );

                        *diffuse = texture_lookup(&tx, coord).unwrap_or(Vec3::ZERO);
                    }
                }

                *illum = *diffuse * t.mtl.ka;

                *normal = match &t.mode {
                    TriNormMode::Flat => t.snorm,
                    TriNormMode::Smooth(normals) => {
                        // smooth shading
                        ((*normals[0] * alpha) + (*normals[1] * tr.b) + (*normals[2] * tr.g)).norm()
                    }
                    TriNormMode::Map(norm) => {
                        // normal mapping
                        texture_lookup(&norm, coord)
                            .unwrap_or(Vec3::new(0.0, 1.0, 0.0))
                            .norm()
                            .transform(t.tbn)
                    }
                }
            }
        },
        None => (),
    }
}

#[inline]
fn setup_eta(
    normal: &mut Vec3,
    vi: Vec3,
    shape: &Shape,
    stack: &mut Vec<f32>,
    eta: f32,
) -> (f32, f32) {
    if normal.dot(vi) < 0.0 {
        *normal = -*normal;

        if stack.len() <= 1 {
            (
                match shape {
                    Shape::Sphere(s) => s.mtl.eta,
                    Shape::Triangle(t) => t.mtl.eta,
                },
                eta,
            )
        } else {
            // leaving a volume -> set eta_i and eta_t accordingly
            (
                stack.pop().unwrap_or(eta),
                stack.last().unwrap_or(&eta).clone(),
            )
        }
    } else {
        // entering volume
        let eta_i: f32 = stack.last().unwrap_or(&eta).clone();
        let eta_t: f32 = match shape {
            Shape::Sphere(s) => s.mtl.eta,
            Shape::Triangle(t) => t.mtl.eta,
        };
        stack.push(eta_t);
        (eta_i, eta_t)
    }
}

#[inline]
fn apply_lighting(
    lights: &Vec<Light>,
    p: Vec3,
    normal: Vec3,
    vi: Vec3,
    diffuse: Vec3,
    shape: &Shape,
    bspheres: &[BoundingSphere],
    shapes: &[Shape],
    illum: &mut Vec3,
) -> () {
    for light in lights {
        let l: Vec3 = if light.point {
            (light.pos - p).norm()
        } else {
            (-light.pos).norm()
        };
        let h: Vec3 = (l + vi).norm();

        // calculate diffuse and specular components
        let (df, sp): (Vec3, Vec3) = match shape {
            Shape::Sphere(s) => (
                diffuse * (s.mtl.kd * f32::max(0.0, normal.dot(l))),
                s.mtl.specular * (s.mtl.ks * f32::max(0.0, f32::powf(normal.dot(h), s.mtl.exp))),
            ),
            Shape::Triangle(t) => (
                diffuse * (t.mtl.kd * f32::max(0.0, normal.dot(l))),
                t.mtl.specular * (t.mtl.ks * f32::max(0.0, f32::powf(normal.dot(h), t.mtl.exp))),
            ),
        };

        let li: Vec3 = (df + sp) * (light.intensity);

        // check if shadow
        let sr: Ray3 = Ray3::new(p, l);

        let light_dist: f32 = if light.point {
            (light.pos - p).mag()
        } else {
            f32::MAX
        };

        // trace ray between p and light
        let st: Trace = trace_ray(sr, bspheres, shapes, Some(shape), light_dist);

        *illum = *illum + (li * st.shadow);
    }
}

#[inline]
fn reflections_transparency(
    p: Vec3,
    normal: Vec3,
    vi: Vec3,
    shape: &Shape,
    eta_i: f32,
    eta_t: f32,
    scene: &Scene,
    acc: usize,
    skip: Option<&Shape>,
    stack: &mut Vec<f32>,
    illum: &mut Vec3,
) -> () {
    let (matte, opacity): (bool, f32) = match shape {
        Shape::Sphere(s) => (s.mtl.ks == 0.0, s.mtl.alpha),
        Shape::Triangle(t) => (t.mtl.ks == 0.0, t.mtl.alpha),
    };
    let f0: f32 = f32::powf((eta_t - eta_i) / (eta_t + eta_i), 2.0);
    let fr: f32 = f0 + (1.0 - f0) * f32::powf(1.0 - vi.dot(normal), 5.0);

    if !matte {
        // reflections
        let refr: Ray3 = Ray3::new(p, (normal * (2.0 * normal.dot(vi))) - vi);
        let reflection: Vec3 = shade_ray(refr, scene, acc + 1, skip, stack);

        *illum = *illum + (reflection * fr);
    }

    if opacity < 1.0 {
        // transparency
        let mut tir = false; // flag for total internal reflection
        let cos_theta_i: f32 = normal.dot(vi);
        let eta_ratio: f32 = eta_t / eta_i;

        if (eta_ratio) < 1.0 {
            // possible tir
            let critical: f32 = f32::asin(eta_ratio);
            if f32::acos(cos_theta_i) >= critical {
                tir = true;
            }
        }

        if !tir {
            let t: Vec3 = ((-normal
                * (f32::sqrt(
                    1.0 - (f32::powf(eta_i / eta_t, 2.0) * (1.0 - f32::powf(cos_theta_i, 2.0))),
                )))
                + (((normal * cos_theta_i) - vi) * (eta_i / eta_t)))
                .norm();
            let transmitted: Vec3 = shade_ray(Ray3::new(p, t), scene, acc + 1, skip, stack);

            *illum = *illum + (transmitted * ((1.0 - fr) * (1.0 - opacity)));
        }
    }
}

#[inline]
pub fn shade_ray(
    r: Ray3,
    scene: &Scene,
    acc: usize,
    skip: Option<&Shape>,
    stack: &mut Vec<f32>,
) -> Vec3 {
    let bkgcolor: Vec3 = scene.bkgcolor;

    // cap on how many times reflection can take place
    if acc >= BOUNCES {
        return bkgcolor;
    }

    let tr: Trace = trace_ray(r, &scene.bspheres, &scene.shapes, skip, -1.0);
    let mut illum: Vec3 = Vec3::ZERO;
    let mut diffuse: Vec3 = Vec3::ZERO;
    let mut normal: Vec3 = Vec3::ZERO;
    let vi: Vec3 = (-r.dir).norm();
    let p: Vec3 = r.origin + (r.dir * tr.t);

    // no intersection
    if tr.shape == None {
        return bkgcolor;
    }

    diffuse_normal(&mut diffuse, &mut normal, &mut illum, p, &tr);
    let shape: &Shape = tr.shape.unwrap(); // must not be None

    let (eta_i, eta_t) = setup_eta(&mut normal, vi, &*shape, stack, scene.eta);

    apply_lighting(
        &scene.lights,
        p,
        normal,
        vi,
        diffuse,
        &*shape,
        &scene.bspheres,
        &scene.shapes,
        &mut illum,
    );

    reflections_transparency(
        p, normal, vi, &*shape, eta_i, eta_t, scene, acc, skip, stack, &mut illum,
    );

    illum.clamp(0.0, 1.0)
}
