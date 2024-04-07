use num::Float;

pub mod broad;
pub mod narrow;

pub mod utility;
pub mod vec2;

static CSCOPE_TOLERANCE_ABS: f64 = 0.0001;
static CSCOPE_TOLERANCE_RELATIVE: f64 = 0.0001;

pub trait NumTolerance: Float {
    fn error_mixed(&self, rhs: Self) -> Self;

    fn error_relative(&self, rhs: Self) -> Self;

    fn error_abs(&self) -> Self;

    fn is_difference_small(&self, rhs: Self) -> bool {
        *self - rhs < self.error_mixed(rhs)
    }

    fn is_difference_trivial(&self, rhs: Self) -> bool {
        (*self - rhs).abs() < self.error_mixed(rhs)
    }

    fn is_trivial_abs(&self) -> bool {
        self.abs() < self.error_abs()
    }
}

impl NumTolerance for f32 {
    fn error_mixed(&self, rhs: Self) -> Self {
        Self::max(
            CSCOPE_TOLERANCE_ABS as f32,
            CSCOPE_TOLERANCE_RELATIVE as f32 * Self::max(self.abs(), rhs.abs()),
        )
    }

    fn error_relative(&self, rhs: Self) -> Self {
        CSCOPE_TOLERANCE_RELATIVE as f32 * Self::max(self.abs(), rhs.abs())
    }

    fn error_abs(&self) -> Self {
        CSCOPE_TOLERANCE_ABS as f32
    }
}

impl NumTolerance for f64 {
    fn error_mixed(&self, rhs: Self) -> Self {
        Self::max(
            CSCOPE_TOLERANCE_ABS,
            CSCOPE_TOLERANCE_RELATIVE * Self::max(self.abs(), rhs.abs()),
        )
    }

    fn error_relative(&self, rhs: Self) -> Self {
        CSCOPE_TOLERANCE_RELATIVE * Self::max(self.abs(), rhs.abs())
    }

    fn error_abs(&self) -> Self {
        CSCOPE_TOLERANCE_ABS
    }
}

#[cfg(test)]
mod test_tolerance {

    use super::{NumTolerance, CSCOPE_TOLERANCE_ABS, CSCOPE_TOLERANCE_RELATIVE};
    use num::Float;

    #[test]
    fn test_absolute_tolerance() {
        assert_eq!(CSCOPE_TOLERANCE_ABS, (-320.0).error_abs());
        assert_eq!(CSCOPE_TOLERANCE_ABS, 10.1.error_abs());
        assert_eq!(CSCOPE_TOLERANCE_ABS, 1100044.5.error_abs());
        assert_eq!(CSCOPE_TOLERANCE_ABS, f64::max_value().error_abs());
        assert_eq!(CSCOPE_TOLERANCE_ABS, f64::min_value().error_abs());
    }

    #[test]
    fn test_relative_tolerance() {
        let small = 0.000001012;
        let mid = -10.501;
        let large = 2132323.4;
        let max = f64::max_value();

        assert_eq!(
            CSCOPE_TOLERANCE_RELATIVE * small,
            small.error_relative(small)
        );
        assert_eq!(
            CSCOPE_TOLERANCE_RELATIVE * mid.abs(),
            small.error_relative(mid)
        );
        assert_eq!(CSCOPE_TOLERANCE_RELATIVE * max, max.error_relative(large));
    }

    #[test]
    fn test_mixed_tolerance() {
        let small = 0.000001012;
        let mid = -10.501;
        let large = 2132323.4;
        let max = f64::max_value();

        assert_eq!(CSCOPE_TOLERANCE_ABS, small.error_mixed(small));
        assert_eq!(
            CSCOPE_TOLERANCE_RELATIVE * mid.abs(),
            small.error_mixed(mid)
        );
        assert_eq!(
            CSCOPE_TOLERANCE_RELATIVE * max.abs(),
            max.error_mixed(large)
        );
    }

    #[test]
    fn test_trivial() {
        assert!((0.0000000000000000001).is_trivial_abs());
        assert!((-0.00000203).is_trivial_abs());
        assert!(!0.1.is_trivial_abs());
        assert!(!0.01.is_difference_trivial(0.011));
        assert!(32.0.is_difference_trivial(32.0000004));
    }

    #[test]
    fn test_small() {
        assert!(0.01.is_difference_small(0.011));
        assert!(32.0.is_difference_small(32.0000004));
        assert!(10.0.is_difference_small(11.0));
        assert!(!11.0.is_difference_small(10.0));
    }
}
