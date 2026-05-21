use crate::entities::shape::*;
use crate::util::vec3::*;

const ERR_BOUND: f32 = 0.001;

pub struct Trace<'a> {
    pub shape: Option<&'a Shape>,
    pub b: f32,
    pub g: f32,
    pub t: f32,
    pub shadow: f32,
}

#[inline]
pub fn trace_ray<'a>(
    r: Ray3,
    shapes: &'a Vec<Shape>,
    skip: Option<&'a Shape>,
    l: f32,
) -> Trace<'a> {
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
                let ep: Vec3 = pt - *(tr.vertices[0]);
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
