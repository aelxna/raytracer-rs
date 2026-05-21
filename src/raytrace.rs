use crate::io::scene::Scene;
use crate::util::light::*;
use crate::util::material::*;
use crate::util::shape::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use anyhow::{Result, anyhow};
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
                    t1 = (-b + f32::sqrt(discrim)) / 2.0;
                    t2 = (-b - f32::sqrt(discrim)) / 2.0;
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
                    (tr.e1.sq_mag() * tr.e2.sq_mag()) - (tr.e1.dot(tr.e2) * tr.e1.dot(tr.e2));
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
fn diffuse_normal(
    diffuse: &mut Vec3,
    normal: &mut Vec3,
    illum: &mut Vec3,
    p: &Vec3,
    tr: &Trace,
) -> () {
    match tr.shape {
        Some(sh) => match sh {
            Shape::Sphere(s) => {
                match s.texture.clone() {
                    None => {
                        *diffuse = s.mtl.diffuse;
                    }
                    Some(tx) => {
                        // replace diffuse with texture lookup
                        let sphere_norm: Vec3 = (*p - s.center) * (1.0 / s.radius);
                        let phi: f32 = f32::acos(sphere_norm.z);
                        let mut theta: f32 = f32::atan2(sphere_norm.y, sphere_norm.x);
                        if theta < 0.0 {
                            theta += 2.0 * PI;
                        }
                        let coord: Vec2 = Vec2::new(theta / (2.0 * PI), phi / PI);

                        *diffuse = texture_lookup(&tx, coord).unwrap_or(Vec3::ZERO);
                    }
                }

                *illum = *diffuse * s.mtl.ka;
                *normal = (*p - s.center).norm();
            }
            Shape::Triangle(t) => {
                let alpha: f32 = 1.0 - (tr.b + tr.g);
                match t.texture.clone() {
                    None => {
                        *diffuse = t.mtl.diffuse;
                    }
                    Some(tx) => {
                        // replace diffuse with texture lookup
                        let texcoords = t.texcoords.unwrap_or([Vec2::ZERO, Vec2::ZERO, Vec2::ZERO]);
                        let coord: Vec2 = Vec2::new(
                            (alpha * texcoords[0].x)
                                + (tr.b * texcoords[1].x)
                                + (tr.g * texcoords[2].x),
                            (alpha * texcoords[0].y)
                                + (tr.b * texcoords[1].y)
                                + (tr.g * texcoords[2].y),
                        );

                        *diffuse = texture_lookup(&tx, coord).unwrap_or(Vec3::ZERO);
                    }
                }

                *illum = *diffuse * t.mtl.ka;

                *normal = match t.normals {
                    None => t.snorm,
                    Some(normals) => {
                        // smooth shading
                        ((normals[0] * alpha) + (normals[1] * tr.b) + (normals[2] * tr.g)).norm()
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
    vi: &Vec3,
    shape: &Shape,
    stack: &mut Vec<f32>,
    eta: f32,
) -> (f32, f32) {
    if normal.dot(*vi) < 0.0 {
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
    p: &Vec3,
    normal: &Vec3,
    vi: &Vec3,
    diffuse: &Vec3,
    shape: &Shape,
    shapes: &Vec<Shape>,
    illum: &mut Vec3,
) -> () {
    for light in lights {
        let l: Vec3 = if light.point {
            (light.pos - *p).norm()
        } else {
            (-light.pos).norm()
        };
        let h: Vec3 = (l + *vi).norm();

        // calculate diffuse and specular components
        let (df, sp): (Vec3, Vec3) = match shape {
            Shape::Sphere(s) => (
                *diffuse * (s.mtl.kd * f32::max(0.0, normal.dot(l))),
                s.mtl.specular * (s.mtl.ks * f32::max(0.0, f32::powf(normal.dot(h), s.mtl.exp))),
            ),
            Shape::Triangle(t) => (
                *diffuse * (t.mtl.kd * f32::max(0.0, normal.dot(l))),
                t.mtl.specular * (t.mtl.ks * f32::max(0.0, f32::powf(normal.dot(h), t.mtl.exp))),
            ),
        };

        let li: Vec3 = (df + sp) * (light.intensity);

        // check if shadow
        let sr: Ray3 = Ray3::new(*p, l);

        let light_dist: f32 = if light.point {
            (light.pos - *p).mag()
        } else {
            f32::MAX
        };

        // trace ray between p and light
        let st: Trace = trace_ray(sr, shapes, Some(shape), light_dist);

        *illum = *illum + (li * st.shadow);
    }
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
    // int matte = 0;
    // float opacity;
    // if (s != NULL) {
    //   if (s->mtl.ks == 0)
    //     matte = 1;
    //   opacity = s->mtl.alpha;
    // } else {
    //   if (t->mtl.ks == 0)
    //     matte = 1;
    //   opacity = t->mtl.alpha;
    // }
    // if (debug_flag) {
    //   printf("alpha = %f\n", opacity);
    // }
    //
    // float f0 = powf((eta_t - eta_i) / (eta_t + eta_i), 2);
    // float fr = f0 + (1 - f0) * powf((1 - dot(vi, normal)), 5);
    // if (debug_flag) {
    //   printf("f0 = %f, fr = %f\n", f0, fr);
    // }
    //
    // if (!matte) { // reflections
    //   ray3_t refr = ray3_new(p, sub(scale(2 * dot(normal, vi), normal), vi));
    //   vec3_t reflection =
    //       shade_ray(refr, c, (acc + 1), tr.shape, stack, debug_flag);
    //
    //   illum = add(illum, scale(fr, reflection));
    // }
    //
    // if (opacity < 1) { // transparency
    //   int tir = 0;     // flag for total internal reflection
    //   float cos_theta_i = dot(normal, vi);
    //   if (debug_flag) {
    //     printf("cos_theta_i = %f, eta_t / eta_i = %f\n", cos_theta_i,
    //            eta_t / eta_i);
    //   }
    //
    //   if ((eta_t / eta_i) < 1) {
    //     if (debug_flag) {
    //       printf("Possible tir!!!!!\n");
    //     }
    //     float critical = asinf(eta_t / eta_i);
    //     if (acosf(cos_theta_i) >= critical) { // total internal reflection
    //       if (debug_flag) {
    //         printf("tir!!!!! theta_i = %f > critical = %f\n",
    //                acosf(cos_theta_i), critical);
    //       }
    //       tir = 1;
    //     }
    //   }
    //   if (!tir) {
    //     vec3_t t = normalize(
    //         add(scale(sqrtf(1.0 - (powf(eta_i / eta_t, 2) *
    //                                (1.0 - powf(cos_theta_i, 2)))),
    //                   negate(normal)),
    //             scale((eta_i / eta_t), sub(scale(cos_theta_i, normal), vi))));
    //     if (debug_flag) {
    //       printf("t = <%f, %f, %f>\n", t.x, t.y, t.z);
    //     }
    //
    //     vec3_t transmitted =
    //         shade_ray(ray3_new(p, t), c, (acc + 1), skip, stack, debug_flag);
    //     if (debug_flag) {
    //       printf("transmitted color = <%f, %f, %f>\n", transmitted.x,
    //              transmitted.y, transmitted.z);
    //     }
    //
    //     illum = add(illum, scale((1.0 - fr) * (1.0 - opacity), transmitted));
    //   }
    // }
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
    if acc >= BOUNCES {
        return bkgcolor;
    }

    let tr: Trace = trace_ray(r, &scene.shapes, skip, -1.0);
    let mut illum: Vec3 = Vec3::ZERO;
    let mut diffuse: Vec3 = Vec3::ZERO;
    let mut normal: Vec3 = Vec3::ZERO;
    let vi: Vec3 = (-r.dir).norm();
    let p: Vec3 = r.origin + (r.dir * tr.t);

    // no intersection
    if tr.shape == None {
        return bkgcolor;
    }
    let shape: &Shape = tr.shape.unwrap(); // must not be None

    diffuse_normal(&mut diffuse, &mut normal, &mut illum, &p, &tr);

    let (eta_i, eta_t) = setup_eta(&mut normal, &vi, shape, stack, scene.eta.unwrap_or(1.0));

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
