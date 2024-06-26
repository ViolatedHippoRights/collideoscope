use num::clamp;
use std::ops::{Add, Neg, Sub};

use crate::NumTolerance;

#[derive(Clone, Copy)]
pub struct Vec2<T: NumTolerance> {
    pub x: T,
    pub y: T,
}

impl<T: NumTolerance> Vec2<T> {
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn length_squared(&self) -> T {
        return (self.x * self.x) + (self.y * self.y);
    }

    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self::new(self.x / length, self.y / length)
    }

    pub fn dot(&self, rhs: Vec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn rotate_counter_90(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn rotate_clock_90(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    pub fn perp(&self, vec: Vec2<T>) -> bool {
        self.dot(vec).is_trivial_abs()
    }

    pub fn scale(&self, s: T) -> Vec2<T> {
        Vec2::new(self.x * s, self.y * s)
    }

    pub fn clamp(&self, min: Self, max: Self) -> Self {
        Vec2::new(
            clamp::<T>(self.x, min.x, max.x),
            clamp::<T>(self.y, min.y, max.y),
        )
    }
}

impl<T: NumTolerance> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: NumTolerance> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: NumTolerance> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod test_vectors {

    use super::Vec2;
    use float_eq::assert_float_eq;

    #[test]
    fn test_ops() {
        let a = Vec2::new(30.0, 10.0);
        let b = Vec2::new(-5.0, 12.0);
        let c = Vec2::new(-1.0, -1.7);

        assert_float_eq!((a + b).x, 25.0, abs <= 0.0001);
        assert_float_eq!((a + b).y, 22.0, abs <= 0.0001);

        assert_float_eq!((-c).x, 1.0, abs <= 0.0001);
        assert_float_eq!((-c).y, 1.7, abs <= 0.0001);

        assert_float_eq!((c - b).x, 4.0, abs <= 0.0001);
        assert_float_eq!((a - c).y, 11.7, abs <= 0.0001);

        assert_float_eq!(a.dot(b), -30.0, abs <= 0.0001);

        assert_float_eq!(a.scale(0.1).x, 3.0, abs <= 0.0001);
        assert_float_eq!(a.scale(0.1).y, 1.0, abs <= 0.0001);
    }

    #[test]
    fn test_rotation() {
        let a = Vec2::new(30.0, 10.0);
        let b = Vec2::new(-5.0, 12.0);
        let c = Vec2::new(-1.0, -1.7);

        assert_float_eq!(a.rotate_clock_90().x, 10.0, abs <= 0.0001);
        assert_float_eq!(a.rotate_clock_90().y, -30.0, abs <= 0.0001);

        assert_float_eq!(b.rotate_counter_90().x, -12.0, abs <= 0.0001);
        assert_float_eq!(b.rotate_counter_90().y, -5.0, abs <= 0.0001);

        assert_float_eq!(c.rotate_clock_90().x, -1.7, abs <= 0.0001);
        assert_float_eq!(c.rotate_counter_90().y, -1.0, abs <= 0.0001);
    }

    #[test]
    fn test_length() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(1.0, -1.0);
        let c = Vec2::new(-3.0, 4.0);

        assert_float_eq!(a.length_squared(), 1.0, abs <= 0.0001);
        assert_float_eq!(a.length(), 1.0, abs <= 0.0001);

        assert_float_eq!(b.length_squared(), 2.0, abs <= 0.0001);
        assert_float_eq!(b.length(), f64::sqrt(2.0), abs <= 0.0001);

        assert_float_eq!(c.length_squared(), 25.0, abs <= 0.0001);
        assert_float_eq!(c.length(), 5.0, abs <= 0.0001);
    }

    #[test]
    fn test_perp() {
        let a = Vec2::new(1.0, 0.0);
        let b = Vec2::new(0.0, -1.0);
        let c = Vec2::new(-3.0, 4.0);
        let d = Vec2::new(-4.0, -3.0);
        let e = Vec2::new(-4.0, -2.99);

        assert!(a.perp(b));
        assert!(!b.perp(c));
        assert!(!a.perp(d));
        assert!(c.perp(d));
        assert!(!e.perp(c));
    }
}
