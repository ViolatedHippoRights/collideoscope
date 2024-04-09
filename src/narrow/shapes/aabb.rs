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
        if let ShapeType::AABB(aabb) = shape.shape() {
            return self
                .aabb_resolution(position, aabb, shape_position)
                .colliding;
        }

        self.sat_collides(position, shape, shape_position)
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
            ShapeType::AABB(aabb) => self.aabb_resolution(position, aabb, shape_position),
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
    use num::Float;

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

    #[test]
    fn test_aabb_aabb_collision() {
        let box0 = AABB::new(10.0, 10.0);
        let box1 = AABB::new(2.0, 4.0);
        let box2 = AABB::new(4.0, 2.0);

        let res0 = box0.collision_resolution(Vec2::new(5.0, 5.0), &box1, Vec2::new(1.0, -1.5));
        assert!(res0.colliding);
        assert_float_eq!(res0.penetration, 0.5, abs <= 0.01);
        assert!(res0.axis.perp(Vec2::new(1.0, 0.0)));
        assert!(res0.axis.y.is_sign_positive());

        let res0_sat =
            box0.sat_collision_resolution(Vec2::new(5.0, 5.0), &box1, Vec2::new(1.0, -1.5));
        assert!(res0_sat.colliding);
        assert_float_eq!(res0_sat.penetration, 0.5, abs <= 0.01);
        assert!(res0_sat.axis.perp(Vec2::new(1.0, 0.0)));
        assert!(res0_sat.axis.y.is_sign_positive());

        let res1 = box1.collision_resolution(Vec2::new(0.0, 0.0), &box2, Vec2::new(2.75, 0.5));
        assert!(res1.colliding);
        assert_float_eq!(res1.penetration, 0.25, abs <= 0.01);
        assert!(res1.axis.perp(Vec2::new(0.0, 1.0)));
        assert!(res1.axis.x.is_sign_negative());

        let res1_sat =
            box1.sat_collision_resolution(Vec2::new(0.0, 0.0), &box2, Vec2::new(2.75, 0.5));
        assert!(res1_sat.colliding);
        assert_float_eq!(res1_sat.penetration, 0.25, abs <= 0.01);
        assert!(res1_sat.axis.perp(Vec2::new(0.0, 1.0)));
        assert!(res1_sat.axis.x.is_sign_negative());

        let res2 = box2.collision_resolution(Vec2::new(0.0, 0.0), &box2, Vec2::new(8.75, 0.5));
        assert!(!res2.colliding);
        assert_float_eq!(res2.penetration, f32::max_value(), abs <= 0.01);
        assert_float_eq!(res2.axis.length_squared(), 0.0, abs <= 0.01);

        let res2_sat =
            box2.sat_collision_resolution(Vec2::new(0.0, 0.0), &box2, Vec2::new(8.75, 0.5));
        assert!(!res2_sat.colliding);
        assert_float_eq!(res2_sat.penetration, f32::max_value(), abs <= 0.01);
        assert_float_eq!(res2_sat.axis.length_squared(), 0.0, abs <= 0.01);

        assert!(!box0.collides(Vec2::new(0.0, 0.0), &box2, Vec2::new(15.0, 3.0)));
        assert!(!box0.sat_collides(Vec2::new(0.0, 0.0), &box2, Vec2::new(15.0, 3.0)));

        assert!(box1.collides(Vec2::new(0.0, 0.0), &box2, Vec2::new(1.0, 1.9)));
        assert!(box1.sat_collides(Vec2::new(0.0, 0.0), &box2, Vec2::new(1.0, 1.9)));
    }
}
