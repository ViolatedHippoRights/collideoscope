use crate::{
    narrow::sat::{Axis, Resolution, SATable},
    utility::{closest_vertex, Projection},
    vec2::Vec2,
    NumTolerance,
};

use super::{ShapeType, Shapeable};

pub struct Capsule<T: NumTolerance> {
    half_path: Vec2<T>,
    radius: T,
}

impl<T: NumTolerance> Capsule<T> {
    pub fn new(half_path: Vec2<T>, radius: T) -> Self {
        Self { half_path, radius }
    }
}

impl<T: NumTolerance> SATable<T> for Capsule<T> {
    fn axes(&self) -> impl Iterator<Item = Axis<T>> {
        [
            Axis::Static {
                vector: self.half_path.rotate_counter_90(),
                normalized: false,
            },
            Axis::Dynamic {
                point: self.half_path,
            },
            Axis::Dynamic {
                point: -self.half_path,
            },
        ]
        .into_iter()
    }

    fn project(&self, axis: Vec2<T>, position: Vec2<T>) -> Projection<T> {
        let dot = axis.dot(position);
        let path_proj = axis.dot(self.half_path).abs();

        let proj = self.radius * axis.length(); // The max and min will occur with parallel vectors, so the dot product is the product of the lengths
        Projection {
            min: dot - path_proj - proj,
            max: dot + path_proj + proj,
        }
    }

    fn axis_from_point(&self, position: Vec2<T>, point: Vec2<T>) -> Vec2<T> {
        closest_vertex(point, position, &[self.half_path, -self.half_path]) - point
    }

    fn collides<S>(&self, position: Vec2<T>, shape: &S, shape_position: Vec2<T>) -> bool
    where
        S: SATable<T> + Shapeable<T>,
    {
        match shape.shape() {
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
            _ => self.sat_collision_resolution(position, shape, shape_position),
        }
    }
}

impl<T: NumTolerance> Shapeable<T> for Capsule<T> {
    fn shape(&self) -> ShapeType<T> {
        ShapeType::Capsule(self)
    }
}

#[cfg(test)]
mod circle_tests {

    use super::Capsule;
    use crate::{
        narrow::sat::{Axis, SATable},
        vec2::Vec2,
    };

    #[test]
    fn test_axes() {
        let cap0 = Capsule::new(Vec2::new(1.0, 0.0), 1.0);
        let cap1 = Capsule::new(Vec2::new(2.0, -1.0), 2.0);

        let pos0 = Vec2::new(3.0, 0.0);
        let pos1 = Vec2::new(-1.0, -1.0);

        for axis in cap0.axes() {
            match axis {
                Axis::Dynamic { point: _point } => {
                    let ax = cap0.axis_from_point(Vec2::zero(), pos0);
                    assert!(ax.perp(Vec2::new(0.0, 1.0)));
                }
                Axis::Static { vector, normalized } => {
                    assert!(!normalized);
                    assert!(Vec2::new(1.0, 0.0).perp(vector));
                }
            }
        }

        for axis in cap1.axes() {
            match axis {
                Axis::Dynamic { point: _point } => {
                    let ax = cap1.axis_from_point(pos0, pos1);
                    assert!(ax.perp(Vec2::new(0.0, 1.0)) || ax.perp(Vec2::new(-2.0, 2.0)));
                }
                Axis::Static { vector, normalized } => {
                    assert!(!normalized);
                    assert!(Vec2::new(2.0, -1.0).perp(vector));
                }
            }
        }
    }

    #[test]
    fn test_contains_point() {
        let cap0 = Capsule::new(Vec2::new(0.0, 2.0), 1.0);
        let cap1 = Capsule::new(Vec2::new(3.0, 2.0), 2.0);

        let pos = Vec2::new(-2.0, 1.0);

        assert!(cap0.contains_point(Vec2::zero(), Vec2::zero()));
        assert!(cap0.contains_point(Vec2::zero(), Vec2::new(0.5, 2.2)));
        assert!(cap0.contains_point(pos, Vec2::new(-2.5, -1.0)));
        assert!(!cap0.contains_point(pos, Vec2::new(-1.5, 3.9)));

        assert!(cap1.contains_point(Vec2::zero(), Vec2::new(3.0, 2.0)));
        assert!(cap1.contains_point(Vec2::zero(), Vec2::new(-1.5, -2.5)));
        assert!(cap1.contains_point(pos, Vec2::new(2.4, 4.4)));
        assert!(!cap1.contains_point(pos, Vec2::new(-6.6, 0.6)));
    }
}
