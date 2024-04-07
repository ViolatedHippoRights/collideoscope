use crate::{vec2::Vec2, NumTolerance};

pub struct Polygon<T: NumTolerance> {
    first: Vec2<T>,
    sides: Vec<Vec2<T>>,
}
