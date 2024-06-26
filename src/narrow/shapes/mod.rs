use crate::NumTolerance;

#[cfg(test)]
use crate::{narrow::sat::Axis, vec2::Vec2};

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

#[cfg(test)]
fn contains_perpendicular<T: NumTolerance>(
    axes: impl Iterator<Item = Axis<T>>,
    perp: Vec2<T>,
) -> bool {
    for axis in axes {
        match axis {
            Axis::Static {
                vector,
                normalized: _,
            } => {
                if vector.dot(perp).is_trivial_abs() {
                    return true;
                }
            }
            Axis::Dynamic { point: _ } => {
                return false;
            }
        }
    }

    false
}
