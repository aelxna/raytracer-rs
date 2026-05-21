use crate::entities::sphere::*;
use crate::entities::triangle::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Triangle(Triangle),
}
