use crate::{
    narrow::{
        sat::{Axis, Resolution, SATable},
        shapes::{ShapeType, Shapeable},
    },
    utility::Projection,
    vec2::Vec2,
    NumTolerance,
};

use super::aabb::AABB;

pub struct Circle<T: NumTolerance> {
    radius: T,
}

impl<T: NumTolerance> Circle<T> {
    pub fn new(radius: T) -> Self {
        Self { radius }
    }

    pub fn aabb_resolution(
        &self,
        position: Vec2<T>,
        shape: &AABB<T>,
        shape_position: Vec2<T>,
        resolve: bool,
    ) -> Resolution<T> {
        let mut resolution = Resolution::new();

        let vertices = shape.vertices();
        let top_left = shape_position + vertices[0];
        let bottom_right = shape_position + vertices[2];
        let closest = Vec2::clamp(&position, top_left, bottom_right);

        let diff = position - closest;
        let dist_square = diff.length_squared();

        if dist_square.is_difference_small(self.radius * self.radius) {
            resolution.colliding = true;

            if resolve {
                if shape.contains_point(shape_position, position) {
                    let left = position.x - top_left.x;
                    let right = bottom_right.x - position.x;
                    let top = position.y - top_left.y;
                    let bottom = bottom_right.y - position.y;

                    let min = left.min(right.min(top.min(bottom)));
                    let (pen, axis) = match min {
                        _ if min == left => (left + self.radius, Vec2::new(-T::one(), T::zero())),
                        _ if min == right => (right + self.radius, Vec2::new(T::one(), T::zero())),
                        _ if min == top => (top + self.radius, Vec2::new(T::zero(), -T::one())),
                        _ => (bottom + self.radius, Vec2::new(T::zero(), T::one())),
                    };

                    resolution.penetration = pen;
                    resolution.axis = axis;
                } else {
                    let dist = T::sqrt(dist_square);
                    resolution.penetration = self.radius - dist;
                    resolution.axis = diff.scale(T::one() / dist);
                }
            }
        }

        resolution
    }

    pub fn circle_resolution(
        &self,
        position: Vec2<T>,
        shape: &Circle<T>,
        shape_position: Vec2<T>,
        resolve: bool,
    ) -> Resolution<T> {
        let mut resolution = Resolution::new();

        let separation = position - shape_position;
        let l_square = separation.length_squared();
        let rad_sum = self.radius + shape.radius;

        if l_square.is_difference_small(rad_sum * rad_sum) {
            resolution.colliding = true;

            if resolve {
                let length = l_square.sqrt();
                resolution.penetration = rad_sum - length;
                resolution.axis = separation.scale(T::one() / length);
            }
        }

        resolution
    }
}

impl<T: NumTolerance> SATable<T> for Circle<T> {
    fn axes(&self) -> impl Iterator<Item = Axis<T>> {
        [Axis::Dynamic {
            point: Vec2::zero(),
        }]
        .into_iter()
    }

    fn project(&self, axis: Vec2<T>, position: Vec2<T>) -> Projection<T> {
        let dot = axis.dot(position);
        let proj = self.radius * axis.length(); // The max and min will occur with parallel vectors, so the dot product is the product of the lengths
        Projection {
            min: dot - proj,
            max: dot + proj,
        }
    }

    fn axis_from_point(&self, position: Vec2<T>, point: Vec2<T>) -> Vec2<T> {
        point - position
    }

    fn collides<S>(&self, position: Vec2<T>, shape: &S, shape_position: Vec2<T>) -> bool
    where
        S: SATable<T> + Shapeable<T>,
    {
        match shape.shape() {
            ShapeType::AABB(aabb) => {
                self.aabb_resolution(position, aabb, shape_position, false)
                    .colliding
            }
            ShapeType::Circle(circ) => {
                self.circle_resolution(position, circ, shape_position, false)
                    .colliding
            }
            _ => self.sat_collides(position, shape, shape_position),
        }
    }

    fn collision_resolution<S>(
        &self,
        position: Vec2<T>,
        shape: &S,
        shape_position: Vec2<T>,
    ) -> Resolution<T>
    where
        S: SATable<T> + Shapeable<T>,
    {
        match shape.shape() {
            ShapeType::AABB(aabb) => self.aabb_resolution(position, aabb, shape_position, true),
            ShapeType::Circle(circ) => self.circle_resolution(position, circ, shape_position, true),
            _ => self.sat_collision_resolution(position, shape, shape_position),
        }
    }
}

impl<T: NumTolerance> Shapeable<T> for Circle<T> {
    fn shape(&self) -> ShapeType<T> {
        ShapeType::Circle(self)
    }
}

#[cfg(test)]
mod circle_tests {

    use float_eq::assert_float_eq;

    use super::Circle;
    use crate::{
        narrow::sat::{Axis, SATable},
        vec2::Vec2,
    };

    #[test]
    fn test_axes() {
        let circ0 = Circle::new(15.0);
        let circ1 = Circle::new(2.0);

        let pos0 = Vec2::new(5.0, 2.0);
        let pos1 = Vec2::new(-1.0, -1.0);

        for axis in circ0.axes() {
            match axis {
                Axis::Dynamic { point } => {
                    let ax = circ0.axis_from_point(pos0 + point, pos1);
                    assert_float_eq!(ax.x, -6.0, abs <= 0.01);
                    assert_float_eq!(ax.y, -3.0, abs <= 0.01);
                }
                Axis::Static {
                    vector: _v,
                    normalized: _n,
                } => panic!("Circle returned static axis!"),
            }
        }

        for axis in circ1.axes() {
            match axis {
                Axis::Dynamic { point } => {
                    let ax = circ1.axis_from_point(pos1 + point, pos0);
                    assert_float_eq!(ax.x, 6.0, abs <= 0.01);
                    assert_float_eq!(ax.y, 3.0, abs <= 0.01);
                }
                Axis::Static {
                    vector: _v,
                    normalized: _n,
                } => panic!("Circle returned static axis!"),
            }
        }
    }

    #[test]
    fn test_contains_point() {
        let circ0 = Circle::new(10.0);
        let circ1 = Circle::new(1.0);

        let pos0 = Vec2::new(-2.0, 1.0);
        let pos1 = Vec2::new(0.0, 0.0);

        assert!(circ0.contains_point(pos0, Vec2::new(3.0, 3.0)));
        assert!(circ0.contains_point(pos0, Vec2::new(-2.0, 10.9)));
        assert!(circ0.contains_point(pos1, Vec2::new(8.1, 0.0)));
        assert!(!circ0.contains_point(pos1, Vec2::new(10.1, 0.0)));

        assert!(circ1.contains_point(pos1, Vec2::new(0.5, -0.5)));
        assert!(circ1.contains_point(pos1, Vec2::new(0.9, -0.01)));
        assert!(circ1.contains_point(pos0, Vec2::new(-2.5, 0.75)));
        assert!(!circ1.contains_point(pos1, Vec2::new(f32::sqrt(2.1), f32::sqrt(2.1))));
    }
}
