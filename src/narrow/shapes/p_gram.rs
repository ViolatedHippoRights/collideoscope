use crate::{
    narrow::sat::{Axis, Resolution, SATable},
    utility::{closest_vertex, project_onto, Projection},
    vec2::Vec2,
    NumTolerance,
};

use super::{ShapeType, Shapeable};

pub struct Pgram<T: NumTolerance> {
    u: Vec2<T>,
    v: Vec2<T>,
}

impl<T: NumTolerance> Pgram<T> {
    pub fn new(u: Vec2<T>, v: Vec2<T>) -> Self {
        Self { u, v }
    }

    pub fn vertices(&self) -> [Vec2<T>; 4] {
        let half = T::one() / (T::one() + T::one());
        let sum = self.u + self.v;
        let diag = self.u - self.v;

        [
            -sum.scale(half),
            diag.scale(half),
            sum.scale(half),
            -diag.scale(half),
        ]
    }
}

impl<T: NumTolerance> SATable<T> for Pgram<T> {
    fn axes(&self) -> impl Iterator<Item = Axis<T>> {
        [
            Axis::Static {
                vector: self.u.rotate_counter_90(),
                normalized: false,
            },
            Axis::Static {
                vector: self.v.rotate_counter_90(),
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

    fn contains_point(&self, position: Vec2<T>, point: Vec2<T>) -> bool {
        let axis = self.u.rotate_counter_90();
        let proj = point.dot(axis);
        let self_proj = self.project(axis, position);

        if !proj.is_between(self_proj.min, self_proj.max) {
            return false;
        }

        let axis = self.v.rotate_counter_90();
        let proj = point.dot(axis);
        let self_proj = self.project(axis, position);

        proj.is_between(self_proj.min, self_proj.max)
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

impl<T: NumTolerance> Shapeable<T> for Pgram<T> {
    fn shape(&self) -> ShapeType<T> {
        ShapeType::Pgram(self)
    }
}

#[cfg(test)]
mod aabb_tests {

    use float_eq::assert_float_eq;

    use super::Pgram;
    use crate::{narrow::sat::SATable, vec2::Vec2};

    #[test]
    fn test_vertices() {
        let gram0 = Pgram::new(Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0));
        let gram1 = Pgram::new(Vec2::new(5.0, 3.0), Vec2::new(-1.0, -4.0));

        let vertices = gram0.vertices();
        assert_float_eq!(vertices[0].x, -1.0, abs <= 0.01);
        assert_float_eq!(vertices[0].y, -1.0, abs <= 0.01);
        assert_float_eq!(vertices[1].x, 1.0, abs <= 0.01);
        assert_float_eq!(vertices[1].y, -1.0, abs <= 0.01);
        assert_float_eq!(vertices[2].x, 1.0, abs <= 0.01);
        assert_float_eq!(vertices[2].y, 1.0, abs <= 0.01);
        assert_float_eq!(vertices[3].x, -1.0, abs <= 0.01);
        assert_float_eq!(vertices[3].y, 1.0, abs <= 0.01);

        let vertices = gram1.vertices();
        assert_float_eq!(vertices[0].x, -2.0, abs <= 0.01);
        assert_float_eq!(vertices[0].y, 0.5, abs <= 0.01);
        assert_float_eq!(vertices[1].x, 3.0, abs <= 0.01);
        assert_float_eq!(vertices[1].y, 3.5, abs <= 0.01);
        assert_float_eq!(vertices[2].x, 2.0, abs <= 0.01);
        assert_float_eq!(vertices[2].y, -0.5, abs <= 0.01);
        assert_float_eq!(vertices[3].x, -3.0, abs <= 0.01);
        assert_float_eq!(vertices[3].y, -3.5, abs <= 0.01);
    }

    #[test]
    fn test_contains_point() {
        let gram0 = Pgram::new(Vec2::new(8.0, 0.0), Vec2::new(0.0, 4.0));
        let gram1 = Pgram::new(Vec2::new(1.0, 2.0), Vec2::new(2.0, -1.0));

        assert!(gram0.contains_point(Vec2::new(2.0, 2.0), Vec2::new(-1.0, 0.5)));
        assert!(gram0.contains_point(Vec2::new(10.0, 0.0), Vec2::new(7.0, 1.5)));
        assert!(!gram0.contains_point(Vec2::new(20.0, 20.0), Vec2::new(-1.0, 0.5)));
        assert!(!gram0.contains_point(Vec2::new(0.0, 0.0), Vec2::new(6.0, 1.0)));

        assert!(gram1.contains_point(Vec2::new(2.0, 2.0), Vec2::new(2.2, 2.0)));
        assert!(gram1.contains_point(Vec2::new(0.0, 8.0), Vec2::new(1.25, 8.4)));
        assert!(!gram1.contains_point(Vec2::new(20.0, 20.0), Vec2::new(-1.0, 0.5)));
        assert!(!gram1.contains_point(Vec2::new(0.0, 0.0), Vec2::new(6.0, 1.0)));
    }
}
