use crate::{vec2::Vec2, NumTolerance};

pub struct Capsule<T: NumTolerance> {
    half_path: Vec2<T>,
    radius: T,
}
