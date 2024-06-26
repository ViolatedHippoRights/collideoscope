use crate::{
    narrow::{
        sat::{Axis, SATable},
        shapes::{ShapeType, Shapeable},
    },
    utility::{closest_vertex, project_onto, Projection},
    vec2::Vec2,
    NumTolerance,
};

pub struct Triangle<T: NumTolerance> {
    first: Vec2<T>,
    second: Vec2<T>,
    third: Vec2<T>,
}

impl<T: NumTolerance> Triangle<T> {
    pub fn new(vertices: &[Vec2<T>; 3]) -> Self {
        Self {
            first: vertices[0],
            second: vertices[1],
            third: vertices[2],
        }
    }

    pub fn vertices(&self) -> [Vec2<T>; 3] {
        [self.first, self.second, self.third]
    }
}

impl<T: NumTolerance> SATable<T> for Triangle<T> {
    fn axes(&self) -> impl Iterator<Item = Axis<T>> {
        [
            Axis::Static {
                vector: (self.second - self.first).rotate_counter_90(),
                normalized: false,
            },
            Axis::Static {
                vector: (self.third - self.second).rotate_counter_90(),
                normalized: false,
            },
            Axis::Static {
                vector: (self.first - self.third).rotate_counter_90(),
                normalized: false,
            },
        ]
        .into_iter()
    }

    fn project(&self, axis: Vec2<T>, position: Vec2<T>) -> Projection<T> {
        project_onto(position, &self.vertices(), axis)
    }

    fn axis_from_point(&self, position: Vec2<T>, point: Vec2<T>) -> Vec2<T> {
        closest_vertex(point, position, &self.vertices()) - point
    }
}

impl<T: NumTolerance> Shapeable<T> for Triangle<T> {
    fn shape(&self) -> ShapeType<T> {
        ShapeType::Triangle(self)
    }
}

#[cfg(test)]
mod triangle_tests {

    use super::Triangle;
    use crate::{
        narrow::{sat::SATable, shapes::contains_perpendicular},
        vec2::Vec2,
    };

    #[test]
    fn test_axes() {
        let tri0 = Triangle::new(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ]);
        let tri1 = Triangle::new(&[
            Vec2::new(0.5, -1.0 / f64::sqrt(12.0)),
            Vec2::new(-0.5, -1.0 / f64::sqrt(12.0)),
            Vec2::new(0.0, 1.0 / f64::sqrt(3.0)),
        ]);

        assert!(contains_perpendicular(tri0.axes(), Vec2::new(1.0, 0.0)));
        assert!(contains_perpendicular(tri0.axes(), Vec2::new(0.0, 1.0)));
        assert!(contains_perpendicular(tri0.axes(), Vec2::new(1.0, -1.0)));

        assert!(contains_perpendicular(tri1.axes(), Vec2::new(1.0, 0.0)));
        assert!(contains_perpendicular(
            tri1.axes(),
            Vec2::new(0.5, f64::sqrt(3.0) * 0.5)
        ));
        assert!(contains_perpendicular(
            tri1.axes(),
            Vec2::new(-0.5, f64::sqrt(3.0) * 0.5)
        ));
    }

    #[test]
    fn test_contains_point() {
        let tri0 = Triangle::new(&[
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ]);
        let tri1 = Triangle::new(&[
            Vec2::new(0.5, -1.0 / f64::sqrt(12.0)),
            Vec2::new(-0.5, -1.0 / f64::sqrt(12.0)),
            Vec2::new(0.0, 1.0 / f64::sqrt(3.0)),
        ]);

        assert!(tri0.contains_point(Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.25)));
        assert!(tri0.contains_point(Vec2::new(1.0, -1.0), Vec2::new(1.1, -0.5)));
        assert!(!tri0.contains_point(Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.51)));
        assert!(!tri0.contains_point(Vec2::new(1.0, -1.0), Vec2::new(0.5, 0.25)));

        assert!(tri1.contains_point(Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)));
        assert!(tri1.contains_point(Vec2::new(1.0, 2.0), Vec2::new(1.25, 1.9)));
        assert!(!tri1.contains_point(Vec2::new(0.0, 1.0), Vec2::new(0.5, 1.0)));
        assert!(!tri1.contains_point(Vec2::new(-10.0, -10.0), Vec2::new(0.0, 0.0)));
    }
}
