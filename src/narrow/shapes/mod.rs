use crate::NumTolerance;

pub mod aabb;
pub mod capsule;
pub mod circle;
pub mod p_gram;
pub mod polygon;
pub mod triangle;

pub enum ShapeType<'a, T: NumTolerance> {
    AABB(&'a aabb::AABB<T>),
    Capsule(&'a capsule::Capsule<T>),
    Circle(&'a circle::Circle<T>),
    Pgram(&'a p_gram::Pgram<T>),
    Polygon(&'a polygon::Polygon<T>),
    Triangle(&'a triangle::Triangle<T>),
    None,
}

pub trait Shapeable<T: NumTolerance> {
    fn shape(&self) -> ShapeType<T> {
        ShapeType::None
    }
}
