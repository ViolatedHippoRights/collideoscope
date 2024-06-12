use collideoscope::{
    narrow::shapes::{aabb::AABB, capsule::Capsule, circle::Circle, triangle::Triangle},
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{test_collides, test_does_not_collide};

#[test]
fn test_circle_aabb_collision() {
    let circ = Circle::new(2.0);
    let box0 = AABB::new(10.0, 3.0);
    let box1 = AABB::new(2.0, 2.0);

    test_collides(
        &circ,
        Vec2::zero(),
        &box0,
        Vec2::new(0.0, 3.3),
        0.2,
        Vec2::new(0.0, -1.0),
    );
    test_collides(
        &circ,
        Vec2::new(0.6, -7.4),
        &box1,
        Vec2::new(3.0, -5.0),
        2.0 - 1.4 * f64::sqrt(2.0),
        Vec2::new(-1.0, -1.0),
    );

    test_does_not_collide(&circ, Vec2::new(0.5, -7.5), &box1, Vec2::new(3.0, -5.0));
    test_does_not_collide(&circ, Vec2::zero(), &box0, Vec2::new(30.0, 15.0));
}

#[test]
fn test_circle_capsule_collision() {
    let cap0 = Capsule::new(Vec2::new(0.0, 3.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(3.0, -4.0), 2.0);

    let circ = Circle::new(1.0);

    test_collides(
        &circ,
        Vec2::new(0.0, 4.8),
        &cap0,
        Vec2::zero(),
        0.2,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &circ,
        Vec2::new(-1.32, -0.74),
        &cap1,
        Vec2::new(1.0, 1.0),
        0.1,
        Vec2::new(-0.8, -0.6),
    );

    test_does_not_collide(&circ, Vec2::new(-1.5, -4.5), &cap0, Vec2::zero());
    test_does_not_collide(&circ, Vec2::new(10.0, 5.0), &cap1, Vec2::new(11.17, 32.5));
}

#[test]
fn test_circle_circle_collision() {
    let circ0 = Circle::new(5.5);
    let circ1 = Circle::new(1.0);

    test_collides(
        &circ0,
        Vec2::zero(),
        &circ1,
        Vec2::new(6.0, 0.0),
        0.5,
        Vec2::new(-1.0, 0.0),
    );
    test_collides(
        &circ1,
        Vec2::new(2.0, 1.0),
        &circ0,
        Vec2::new(2.0 + 6.0 / f32::sqrt(2.0), 1.0 + 6.0 / f32::sqrt(2.0)),
        0.5,
        Vec2::new(-1.0, -1.0),
    );

    test_does_not_collide(
        &circ1,
        Vec2::new(2.0, 1.0),
        &circ1,
        Vec2::new(2.0 + 6.0 / f32::sqrt(2.0), 1.0 + 6.0 / f32::sqrt(2.0)),
    );
    test_does_not_collide(&circ0, Vec2::new(10.0, 5.0), &circ1, Vec2::zero());
}

#[test]
fn test_circle_pgram_collision() {}

#[test]
fn test_circle_polygon_collision() {}

#[test]
fn test_circle_triangle_collision() {
    let circ = Circle::new(2.0);
    let tri0 = Triangle::new(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ]);
    let tri1 = Triangle::new(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(-3.0, -0.5),
    ]);

    test_collides(
        &circ,
        Vec2::new(-1.9, 0.5),
        &tri0,
        Vec2::zero(),
        0.1,
        Vec2::new(-1.0, 0.0),
    );
    test_collides(
        &circ,
        Vec2::new(2.8, 0.7),
        &tri1,
        Vec2::zero(),
        0.2,
        Vec2::new(1.0, 0.0),
    );

    test_does_not_collide(&circ, Vec2::new(2.0, 4.01), &tri1, Vec2::new(2.0, 1.0));
    test_does_not_collide(&circ, Vec2::zero(), &tri0, Vec2::new(5.0, 5.0));
}
