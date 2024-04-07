use crate::vec2::Vec2;
use num::Float;

pub struct Projection<T: Float> {
    pub min: T,
    pub max: T,
}

impl<T: Float> Projection<T> {
    pub fn new() -> Self {
        Self {
            min: T::max_value(),
            max: T::min_value(),
        }
    }
}

pub fn project_onto<T: Float>(
    position: Vec2<T>,
    vertices: &[Vec2<T>],
    axis: Vec2<T>,
) -> Projection<T> {
    let mut projection = Projection::new();

    for vertex in vertices.iter() {
        let proj = axis.dot(position + *vertex);

        projection.min = T::min(projection.min, proj);
        projection.max = T::max(projection.max, proj);
    }

    projection
}

pub fn closest_vertex<T: Float>(
    point: Vec2<T>,
    position: Vec2<T>,
    vertices: &[Vec2<T>],
) -> Vec2<T> {
    let mut closest = Vec2::zero();
    let mut min = T::max_value();

    for vertex in vertices.iter() {
        let d_s = ((position + *vertex) - point).length_squared();

        if d_s < min {
            closest = *vertex + position;
            min = d_s;
        }
    }

    closest
}

#[cfg(test)]
mod test_utility {

    use float_eq::assert_float_eq;

    use super::{closest_vertex, project_onto};
    use crate::vec2::Vec2;

    #[test]
    fn test_project() {
        let vertices = vec![
            Vec2::zero(),
            Vec2::new(2.0, 1.0),
            Vec2::new(-2.0, -3.0),
            Vec2::new(1.5, -2.5),
            Vec2::new(10.0, 0.0),
        ];

        let proj0 = project_onto(Vec2::zero(), &vertices, Vec2::new(0.0, 1.0));
        assert_float_eq!(proj0.min, -3.0, abs <= 0.0001);
        assert_float_eq!(proj0.max, 1.0, abs <= 0.0001);

        let proj1 = project_onto(Vec2::new(2.0, 1.0), &vertices, Vec2::new(-2.0, 0.0));
        assert_float_eq!(proj1.min, -24.0, abs <= 0.0001);
        assert_float_eq!(proj1.max, 0.0, abs <= 0.0001);

        let proj2 = project_onto(Vec2::new(1.0, 1.0), &vertices, Vec2::new(-1.0, 2.0));
        assert_float_eq!(proj2.min, -9.0, abs <= 0.0001);
        assert_float_eq!(proj2.max, 1.0, abs <= 0.0001);
    }

    #[test]
    fn test_closest() {
        let vertices = vec![
            Vec2::zero(),
            Vec2::new(2.0, 1.0),
            Vec2::new(-2.0, -3.0),
            Vec2::new(1.5, -2.5),
            Vec2::new(10.0, 0.0),
        ];

        assert_float_eq!(
            closest_vertex(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), &vertices).x,
            0.0,
            abs <= 0.0001
        );
        assert_float_eq!(
            closest_vertex(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), &vertices).y,
            1.0,
            abs <= 0.0001
        );
        assert_float_eq!(
            closest_vertex(Vec2::new(2.0, -1.0), Vec2::new(-1.0, 1.0), &vertices).x,
            0.5,
            abs <= 0.0001
        );
        assert_float_eq!(
            closest_vertex(Vec2::new(2.0, -1.0), Vec2::new(-1.0, 1.0), &vertices).y,
            -1.5,
            abs <= 0.0001
        );
    }
}
