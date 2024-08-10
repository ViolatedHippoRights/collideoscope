use crate::{narrow::shapes::Shapeable, utility::Projection, vec2::Vec2, NumTolerance};

pub enum Axis<T: NumTolerance> {
    Static { vector: Vec2<T>, normalized: bool },
    Dynamic { point: Vec2<T> },
}

pub trait SATable<T: NumTolerance> {
    fn axes(&self) -> impl Iterator<Item = Axis<T>>;

    fn project(&self, axis: Vec2<T>, position: Vec2<T>) -> Projection<T>;

    fn axis_from_point(&self, position: Vec2<T>, point: Vec2<T>) -> Vec2<T>;

    fn contains_point(&self, position: Vec2<T>, point: Vec2<T>) -> bool {
        let axes = self.axes();
        for axis in axes {
            let axis_vector = match axis {
                Axis::Static {
                    vector,
                    normalized: _normalized,
                } => vector,
                Axis::Dynamic { point: axis_point } => point - (position + axis_point),
            };

            if !axis_vector.length_squared().is_trivial_abs() {
                let proj = self.project(axis_vector, position);
                let point_proj = axis_vector.dot(point);

                println!(
                    "{}, {}, {}, {}, {}",
                    axis_vector.x, axis_vector.y, proj.min, proj.max, point_proj
                );

                if point_proj.is_difference_small(proj.min)
                    || proj.max.is_difference_small(point_proj)
                {
                    return false;
                }
            }
        }

        return true;
    }

    fn collides<S>(&self, position: Vec2<T>, shape: &S, shape_position: Vec2<T>) -> bool
    where
        S: SATable<T> + Shapeable<T>,
    {
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
        self.sat_collision_resolution(position, shape, shape_position)
    }

    fn sat_collides(
        &self,
        position: Vec2<T>,
        shape: &impl SATable<T>,
        shape_position: Vec2<T>,
    ) -> bool {
        if !half_sat_resolution(self, position, shape, shape_position, false).colliding
            || !half_sat_resolution(shape, shape_position, self, position, false).colliding
        {
            return false;
        }

        true
    }

    fn sat_collision_resolution(
        &self,
        position: Vec2<T>,
        shape: &impl SATable<T>,
        shape_position: Vec2<T>,
    ) -> Resolution<T> {
        let resolution = half_sat_resolution(self, position, shape, shape_position, true);

        if !resolution.colliding {
            return resolution;
        }

        let flipped = half_sat_resolution(shape, shape_position, self, position, true).flipped();

        if !flipped.colliding {
            return flipped;
        }

        match resolution.penetration < flipped.penetration {
            true => resolution,
            false => flipped,
        }
    }
}

pub struct Resolution<T: NumTolerance> {
    pub colliding: bool,
    pub penetration: T,
    pub axis: Vec2<T>,
}

impl<T: NumTolerance> Resolution<T> {
    pub fn new() -> Self {
        Self {
            colliding: false,
            penetration: T::max_value(),
            axis: Vec2::<T>::zero(),
        }
    }

    pub fn flipped(mut self) -> Self {
        self.axis = -self.axis;

        self
    }
}

fn half_sat_resolution<T>(
    actor: &(impl SATable<T> + ?Sized),
    actor_position: Vec2<T>,
    pushed: &(impl SATable<T> + ?Sized),
    pushed_position: Vec2<T>,
    accurate: bool,
) -> Resolution<T>
where
    T: NumTolerance,
{
    let mut resolution = Resolution::new();
    let direction = actor_position - pushed_position;

    let axes = actor.axes();
    for axis in axes {
        let axis_vector = generate_axis(axis, actor_position, pushed, pushed_position, accurate);

        let actor_proj = actor.project(axis_vector, actor_position);
        let pushed_proj = pushed.project(axis_vector, pushed_position);

        if actor_proj.max.is_difference_small(pushed_proj.min)
            || pushed_proj.max.is_difference_small(actor_proj.min)
        {
            return resolution;
        }

        let actor_pen = actor_proj.max - pushed_proj.min;
        let pushed_pen = pushed_proj.max - actor_proj.min;
        let penetration = T::min(actor_pen, pushed_pen);

        if penetration < resolution.penetration {
            resolution.penetration = penetration;
            resolution.axis = axis_vector;
        }
    }

    resolution.colliding = true;
    if resolution.axis.dot(direction).is_sign_negative() {
        resolution.axis = -resolution.axis;
    }

    resolution
}

fn generate_axis<T>(
    raw: Axis<T>,
    actor_position: Vec2<T>,
    pushed: &(impl SATable<T> + ?Sized),
    pushed_position: Vec2<T>,
    accurate: bool,
) -> Vec2<T>
where
    T: NumTolerance,
{
    match raw {
        Axis::Static { vector, normalized } => match accurate {
            true => match normalized {
                true => vector,
                false => vector.normalized(),
            },
            false => vector,
        },
        Axis::Dynamic { point } => {
            let vector = pushed.axis_from_point(pushed_position, actor_position + point);

            match accurate {
                true => vector.normalized(),
                false => vector,
            }
        }
    }
}
