use crate::entities::shape::sphere::*;
use crate::entities::shape::triangle::*;

pub mod sphere;
pub mod triangle;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
    Triangle(Triangle),
}
