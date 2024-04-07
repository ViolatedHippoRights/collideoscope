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
