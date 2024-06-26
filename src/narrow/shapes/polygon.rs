use crate::{
    narrow::sat::{Axis, SATable},
    utility::{closest_vertex, project_onto, Projection},
    vec2::Vec2,
    NumTolerance,
};

use super::{ShapeType, Shapeable};

pub struct Polygon<T: NumTolerance> {
    vertices: Vec<Vec2<T>>,
}

#[derive(Debug)]
pub enum PolygonError {
    TooFewVerticesError,
    ConcaveError,
}

fn concave<T: NumTolerance>(vertices: &[Vec2<T>]) -> bool {
    if vertices.len() <= 2 {
        return false;
    }

    let axis = (vertices[0] - *vertices.last().unwrap()).rotate_clock_90();
    let next = vertices[1] - vertices[0];
    let left = axis.dot(next).is_sign_positive();

    for i in 2..vertices.len() {
        let axis = (vertices[i - 1] - vertices[i - 2]).rotate_clock_90();
        let next = vertices[i] - vertices[i - 1];
        let is_left = axis.dot(next).is_sign_positive();

        if left != is_left {
            return true;
        }
    }

    return false;
}

impl<T: NumTolerance> Polygon<T> {
    pub fn new(vertices: Vec<Vec2<T>>) -> Result<Polygon<T>, PolygonError> {
        if vertices.len() <= 2 {
            return Err(PolygonError::TooFewVerticesError);
        }

        if concave(&vertices) {
            return Err(PolygonError::ConcaveError);
        }

        Ok(Polygon { vertices })
    }

    pub fn vertices(&self) -> &[Vec2<T>] {
        &self.vertices
    }
}

impl<T: NumTolerance> SATable<T> for Polygon<T> {
    fn axes(&self) -> impl Iterator<Item = Axis<T>> {
        let mut axes = Vec::new();

        axes.push(Axis::Static {
            vector: (self.vertices[0] - *self.vertices.last().unwrap()).rotate_counter_90(),
            normalized: false,
        });
        for i in 1..self.vertices.len() {
            axes.push(Axis::Static {
                vector: (self.vertices[i] - self.vertices[i - 1]).rotate_counter_90(),
                normalized: false,
            });
        }

        axes.into_iter()
    }

    fn project(&self, axis: Vec2<T>, position: Vec2<T>) -> Projection<T> {
        project_onto(position, &self.vertices(), axis)
    }

    fn axis_from_point(&self, position: Vec2<T>, point: Vec2<T>) -> Vec2<T> {
        closest_vertex(point, position, &self.vertices()) - point
    }
}

impl<T: NumTolerance> Shapeable<T> for Polygon<T> {
    fn shape(&self) -> ShapeType<T> {
        ShapeType::Polygon(self)
    }
}

#[cfg(test)]
mod triangle_tests {

    use crate::{
        narrow::{sat::SATable, shapes::contains_perpendicular},
        vec2::Vec2,
    };

    use super::{concave, Polygon};

    #[test]
    fn test_concave() {
        assert!(!concave(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0)
        ]));
        assert!(!concave(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 0.0)
        ]));
        assert!(!concave(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.5, 2.0),
            Vec2::new(0.0, 1.0)
        ]));

        assert!(concave(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0)
        ]));
        assert!(concave(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(2.0, 0.0)
        ]));
        assert!(concave(&vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.5, 2.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0)
        ]));
    }

    #[test]
    fn test_constructor() {
        let none = Polygon::<f64>::new(Vec::new());
        let point = Polygon::new(vec![Vec2::new(0.0, 0.0)]);
        let segment = Polygon::new(vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)]);
        let triangle = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ]);
        let concave = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(0.0, 2.0),
        ]);

        assert!(triangle.is_ok());

        assert!(none.is_err());
        assert!(point.is_err());
        assert!(segment.is_err());
        assert!(concave.is_err());
    }

    #[test]
    fn test_axes() {
        let triangle = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ])
        .unwrap();
        let pentagon = Polygon::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.5, 2.0),
            Vec2::new(0.0, 1.0),
        ])
        .unwrap();

        assert!(contains_perpendicular(triangle.axes(), Vec2::new(0.0, 1.0)));
        assert!(contains_perpendicular(triangle.axes(), Vec2::new(1.0, 0.0)));
        assert!(contains_perpendicular(
            triangle.axes(),
            Vec2::new(1.0, -1.0)
        ));

        assert!(contains_perpendicular(pentagon.axes(), Vec2::new(0.0, 1.0)));
        assert!(contains_perpendicular(pentagon.axes(), Vec2::new(1.0, 0.0)));
        assert!(contains_perpendicular(
            pentagon.axes(),
            Vec2::new(-0.5, 1.0)
        ));
        assert!(contains_perpendicular(pentagon.axes(), Vec2::new(0.5, 1.0)))
    }

    #[test]
    fn test_contains_point() {
        let square = Polygon::new(vec![
            Vec2::new(-2.0, -2.0),
            Vec2::new(2.0, -2.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(-2.0, 2.0),
        ])
        .unwrap();
        let pentagon = Polygon::new(vec![
            Vec2::new(-1.0, -1.0),
            Vec2::new(1.0, -1.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(-2.0, 0.0),
        ])
        .unwrap();

        assert!(square.contains_point(Vec2::zero(), Vec2::zero()));
        assert!(square.contains_point(Vec2::zero(), Vec2::new(1.8, 1.8)));
        assert!(square.contains_point(Vec2::new(1.0, 2.0), Vec2::new(-0.5, 2.5)));
        assert!(square.contains_point(Vec2::new(10.0, 5.0), Vec2::new(10.1, 4.9)));
        assert!(!square.contains_point(Vec2::zero(), Vec2::new(3.0, 0.0)));
        assert!(!square.contains_point(Vec2::zero(), Vec2::new(2.1, 1.9)));
        assert!(!square.contains_point(Vec2::new(1.0, 2.0), Vec2::new(-3.0, -2.5)));
        assert!(!square.contains_point(Vec2::new(10.0, 5.0), Vec2::zero()));

        assert!(pentagon.contains_point(Vec2::zero(), Vec2::zero()));
        assert!(pentagon.contains_point(Vec2::zero(), Vec2::new(1.9, 0.0)));
        assert!(pentagon.contains_point(Vec2::new(2.0, 2.0), Vec2::new(1.5, 2.25)));
        assert!(pentagon.contains_point(Vec2::new(1.0, 50.0), Vec2::new(1.0, 50.9)));
        assert!(!pentagon.contains_point(Vec2::zero(), Vec2::new(2.1, 0.0)));
        assert!(!pentagon.contains_point(Vec2::zero(), Vec2::new(2.1, 1.9)));
        assert!(!pentagon.contains_point(Vec2::new(1.0, 2.0), Vec2::new(-3.0, -2.5)));
        assert!(!pentagon.contains_point(Vec2::new(10.0, 5.0), Vec2::zero()));
    }
}
