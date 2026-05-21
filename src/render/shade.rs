use crate::config::scene::Scene;
use crate::entities::light::*;
use crate::entities::shape::*;
use crate::render::trace::*;
use crate::util::material::*;
use crate::util::vec2::*;
use crate::util::vec3::*;
use std::f32::consts::PI;

const BOUNCES: usize = 10;

#[inline]
fn diffuse_normal(
    diffuse: &mut Vec3,
    normal: &mut Vec3,
    illum: &mut Vec3,
    p: Vec3,
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
                        let sphere_norm: Vec3 = (p - s.center) * (1.0 / s.radius);
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
                *normal = (p - s.center).norm();
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
    shapes: &Vec<Shape>,
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
        let st: Trace = trace_ray(sr, shapes, Some(shape), light_dist);

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

    diffuse_normal(&mut diffuse, &mut normal, &mut illum, p, &tr);

    let (eta_i, eta_t) = setup_eta(&mut normal, vi, shape, stack, scene.eta.unwrap_or(1.0));

    apply_lighting(
        &scene.lights,
        p,
        normal,
        vi,
        diffuse,
        shape,
        &scene.shapes,
        &mut illum,
    );

    illum.clamp(0.0, 1.0)
}
