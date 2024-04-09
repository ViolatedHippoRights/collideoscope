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

    use float_eq::assert_float_eq;

    use super::Triangle;
    use crate::{
        narrow::sat::{Axis, SATable},
        vec2::Vec2,
        NumTolerance,
    };

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

    #[test]
    fn test_triangle_triangle_collision() {
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

        let res0 = tri0.collision_resolution(Vec2::new(-0.5, 0.0), &tri1, Vec2::new(0.0, -0.5));
        assert!(res0.colliding);
        assert_float_eq!(res0.penetration, (1.0 / f64::sqrt(3.0)) - 0.5, abs <= 0.01);
        assert!(res0.axis.perp(Vec2::new(1.0, 0.0)));
        assert!(res0.axis.y.is_sign_positive());

        let res0_sat =
            tri0.sat_collision_resolution(Vec2::new(-0.5, 0.0), &tri1, Vec2::new(0.0, -0.5));
        assert!(res0_sat.colliding);
        assert_float_eq!(
            res0_sat.penetration,
            (1.0 / f64::sqrt(3.0)) - 0.5,
            abs <= 0.01
        );
        assert!(res0_sat.axis.perp(Vec2::new(1.0, 0.0)));
        assert!(res0_sat.axis.y.is_sign_positive());

        let res1 = tri1.collision_resolution(Vec2::new(0.75, 0.75), &tri0, Vec2::new(0.0, 0.0));

        assert!(res1.colliding);
        assert_float_eq!(
            res1.penetration,
            (Vec2::new(f64::sqrt(3.0) * 0.25, 1.0 - f64::sqrt(3.0) * 0.25)
                - Vec2::new(0.25, 0.75 - 1.0 / f64::sqrt(12.0)))
            .length(),
            abs <= 0.01
        );
        assert!(res1.axis.perp(Vec2::new(1.0, -1.0)));
        assert!(res1.axis.y.is_sign_positive());

        let res1_sat =
            tri1.sat_collision_resolution(Vec2::new(0.75, 0.75), &tri0, Vec2::new(0.0, 0.0));
        assert!(res1_sat.colliding);
        assert_float_eq!(
            res1_sat.penetration,
            (Vec2::new(f64::sqrt(3.0) * 0.25, 1.0 - f64::sqrt(3.0) * 0.25)
                - Vec2::new(0.25, 0.75 - 1.0 / f64::sqrt(12.0)))
            .length(),
            abs <= 0.01
        );
        assert!(res1_sat.axis.perp(Vec2::new(1.0, -1.0)));
        assert!(res1_sat.axis.y.is_sign_positive());
    }
}
