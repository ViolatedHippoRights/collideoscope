use crate::{vec2::Vec2, NumTolerance};

pub struct Wedge<T: NumTolerance> {
    start: Vec2<T>,
    end: Vec2<T>,
    radius: T,
}
