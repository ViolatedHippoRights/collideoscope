use crate::{
    narrow::{
        sat::{Axis, Resolution, SATable},
        shapes::{ShapeType, Shapeable},
    },
    utility::{closest_vertex, project_onto, Projection},
    vec2::Vec2,
    NumTolerance,
};

pub struct AABB<T: NumTolerance> {
    width: T,
    height: T,
}

impl<T: NumTolerance> AABB<T> {
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    pub fn vertices(&self) -> [Vec2<T>; 4] {
        let two = T::one() + T::one();

        [
            Vec2::new(-self.width / two, -self.height / two),
            Vec2::new(self.width / two, -self.height / two),
            Vec2::new(self.width / two, self.height / two),
            Vec2::new(-self.width / two, self.height / two),
        ]
    }

    pub fn aabb_resolution(
        &self,
        position: Vec2<T>,
        shape: &AABB<T>,
        shape_position: Vec2<T>,
        resolve: bool,
    ) -> Resolution<T> {
        let mut resolution = Resolution::new();

        let two = T::one() + T::one();

        let x_self_max = position.x + self.width / two;
        let x_shape_min = shape_position.x - shape.width / two;
        if x_self_max.is_difference_small(x_shape_min) {
            return resolution;
        }

        let x_shape_max = shape_position.x + shape.width / two;
        let x_self_min = position.x - self.width / two;
        if x_shape_max.is_difference_small(x_self_min) {
            return resolution;
        }

        let y_self_max = position.y + self.height / two;
        let y_shape_min = shape_position.y - shape.height / two;
        if y_self_max.is_difference_small(y_shape_min) {
            return resolution;
        }

        let y_shape_max = shape_position.y + shape.height / two;
        let y_self_min = position.y - self.height / two;
        if y_shape_max.is_difference_small(y_self_min) {
            return resolution;
        }

        resolution.colliding = true;

        if resolve {
            let x_pen = T::min(x_self_max - x_shape_min, x_shape_max - x_self_min);
            let y_pen = T::min(y_self_max - y_shape_min, y_shape_max - y_self_min);

            match y_pen < x_pen {
                true => {
                    resolution.penetration = y_pen;
                    resolution.axis = match position.y < shape_position.y {
                        true => Vec2::new(T::zero(), -T::one()),
                        false => Vec2::new(T::zero(), T::one()),
                    };
                }
                false => {
                    resolution.penetration = x_pen;
                    resolution.axis = match position.x < shape_position.x {
                        true => Vec2::new(-T::one(), T::zero()),
                        false => Vec2::new(T::one(), T::zero()),
                    };
                }
            }
        }

        resolution
    }
}

impl<T: NumTolerance> SATable<T> for AABB<T> {
    fn axes(&self) -> impl Iterator<Item = Axis<T>> {
        [
            Axis::Static {
                vector: Vec2::new(T::one(), T::zero()),
                normalized: true,
            },
            Axis::Static {
                vector: Vec2::new(T::zero(), T::one()),
                normalized: true,
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
        let two = T::one() + T::one();

        if (position.x - point.x).abs() > self.width / two {
            return false;
        }

        (position.y - point.y).abs() <= self.height / two
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
            ShapeType::Circle(circle) => {
                circle
                    .aabb_resolution(shape_position, self, position, false)
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
            ShapeType::Circle(circle) => circle
                .aabb_resolution(shape_position, self, position, true)
                .flipped(),
            _ => self.sat_collision_resolution(position, shape, shape_position),
        }
    }
}

impl<T: NumTolerance> Shapeable<T> for AABB<T> {
    fn shape(&self) -> ShapeType<T> {
        ShapeType::AABB(self)
    }
}

#[cfg(test)]
mod aabb_tests {

    use float_eq::assert_float_eq;

    use super::AABB;
    use crate::{narrow::sat::SATable, vec2::Vec2};

    #[test]
    fn test_vertices() {
        let box0 = AABB::new(10.0, 12.0);
        let box1 = AABB::new(5.0, 5.0);

        let vertices = box0.vertices();
        assert_float_eq!(vertices[0].x, -5.0, abs <= 0.01);
        assert_float_eq!(vertices[0].y, -6.0, abs <= 0.01);
        assert_float_eq!(vertices[1].x, 5.0, abs <= 0.01);
        assert_float_eq!(vertices[1].y, -6.0, abs <= 0.01);
        assert_float_eq!(vertices[2].x, 5.0, abs <= 0.01);
        assert_float_eq!(vertices[2].y, 6.0, abs <= 0.01);
        assert_float_eq!(vertices[3].x, -5.0, abs <= 0.01);
        assert_float_eq!(vertices[3].y, 6.0, abs <= 0.01);

        let vertices = box1.vertices();
        assert_float_eq!(vertices[0].x, -2.5, abs <= 0.01);
        assert_float_eq!(vertices[0].y, -2.5, abs <= 0.01);
        assert_float_eq!(vertices[1].x, 2.5, abs <= 0.01);
        assert_float_eq!(vertices[1].y, -2.5, abs <= 0.01);
        assert_float_eq!(vertices[2].x, 2.5, abs <= 0.01);
        assert_float_eq!(vertices[2].y, 2.5, abs <= 0.01);
        assert_float_eq!(vertices[3].x, -2.5, abs <= 0.01);
        assert_float_eq!(vertices[3].y, 2.5, abs <= 0.01);
    }

    #[test]
    fn test_contains_point() {
        let box0 = AABB::new(8.0, 4.0);
        let box1 = AABB::new(1.0, 16.0);

        assert!(box0.contains_point(Vec2::new(2.0, 2.0), Vec2::new(-1.0, 0.5)));
        assert!(box0.contains_point(Vec2::new(10.0, 0.0), Vec2::new(7.0, 1.5)));
        assert!(!box0.contains_point(Vec2::new(20.0, 20.0), Vec2::new(-1.0, 0.5)));
        assert!(!box0.contains_point(Vec2::new(0.0, 0.0), Vec2::new(6.0, 1.0)));

        assert!(box1.contains_point(Vec2::new(2.0, 2.0), Vec2::new(1.75, 3.5)));
        assert!(box1.contains_point(Vec2::new(0.0, 8.0), Vec2::new(0.25, 15.0)));
        assert!(!box1.contains_point(Vec2::new(20.0, 20.0), Vec2::new(-1.0, 0.5)));
        assert!(!box1.contains_point(Vec2::new(0.0, 0.0), Vec2::new(6.0, 1.0)));
    }
}
