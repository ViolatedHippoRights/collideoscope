use collideoscope::{
    narrow::shapes::{
        aabb::AABB, capsule::Capsule, circle::Circle, p_gram::Pgram, triangle::Triangle,
    },
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{test_collides, test_does_not_collide};

#[test]
fn test_capsule_aabb_collision() {
    let cap0 = Capsule::new(Vec2::new(0.0, 2.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(2.0, 2.0), 2.0);

    let box0 = AABB::new(10.0, 5.0);
    let box1 = AABB::new(2.0, 1.0);

    test_collides(
        &cap0,
        Vec2::zero(),
        &box0,
        Vec2::new(-5.9, -2.4),
        0.1,
        Vec2::new(1.0, 0.0),
    );
    test_collides(
        &cap1,
        Vec2::new(1.0, -2.0),
        &box1,
        Vec2::new(0.0, 0.4),
        2.0 - f64::sqrt(2.0) + (0.1 / f64::sqrt(2.0)),
        Vec2::new(1.0, -1.0),
    );

    test_does_not_collide(&cap0, Vec2::new(2.0, 1.0), &box0, Vec2::new(7.9, 6.4));
    test_does_not_collide(&cap1, Vec2::zero(), &box1, Vec2::new(30.0, 61.0));
}

#[test]
fn test_capsule_capsule_collision() {
    let cap0 = Capsule::new(Vec2::new(0.0, 1.0), 2.0);
    let cap1 = Capsule::new(Vec2::new(2.0, -3.0), 1.0);
    let cap2 = Capsule::new(Vec2::new(1.0, 1.0), 3.0);

    test_collides(
        &cap0,
        Vec2::zero(),
        &cap2,
        Vec2::new(-2.9, 3.9),
        5.0 - 2.9 * f64::sqrt(2.0),
        Vec2::new(1.0, -1.0),
    );
    test_collides(
        &cap1,
        Vec2::new(6.5, -4.0),
        &cap0,
        Vec2::new(2.0, -1.0),
        0.5,
        Vec2::new(1.0, 0.0),
    );

    test_does_not_collide(&cap0, Vec2::new(2.0, 1.0), &cap1, Vec2::new(6.0, 3.0));
    test_does_not_collide(&cap2, Vec2::new(10.0, 0.0), &cap0, Vec2::new(0.0, 10.0));
}

#[test]
fn test_capsule_circle_collision() {
    let cap0 = Capsule::new(Vec2::new(2.0, 2.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(3.0, 0.0), 2.0);

    let circ = Circle::new(1.0);

    test_collides(
        &cap0,
        Vec2::zero(),
        &circ,
        Vec2::new(-2.0, -3.8),
        0.2,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &cap1,
        Vec2::new(0.0, 1.0),
        &circ,
        Vec2::new(2.0, 3.7),
        0.3,
        Vec2::new(0.0, -1.0),
    );

    test_does_not_collide(&cap0, Vec2::new(1.0, -1.0), &circ, Vec2::new(4.5, 2.5));
    test_does_not_collide(&cap1, Vec2::new(32.0, 5.0), &circ, Vec2::zero());
}

#[test]
fn test_capsule_pgram_collision() {
    let cap0 = Capsule::new(Vec2::new(0.0, 2.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(2.0, 2.0), 2.0);

    let gram0 = Pgram::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0));
    let gram1 = Pgram::new(Vec2::new(2.0, 0.0), Vec2::new(0.0, 1.0));

    test_collides(
        &cap0,
        Vec2::zero(),
        &gram0,
        Vec2::new(-1.9, 0.0),
        0.1,
        Vec2::new(1.0, 0.0),
    );
    test_collides(
        &cap1,
        Vec2::new(1.0, -2.0),
        &gram1,
        Vec2::new(0.0, 0.4),
        2.0 - f64::sqrt(2.0) + (0.1 / f64::sqrt(2.0)),
        Vec2::new(1.0, -1.0),
    );

    test_does_not_collide(&cap0, Vec2::new(2.0, 10.0), &gram0, Vec2::new(4.1, 11.0));
    test_does_not_collide(&cap1, Vec2::zero(), &gram1, Vec2::new(30.0, 61.0));
}

#[test]
fn test_capsule_polygon_collision() {}

#[test]
fn test_capsule_triangle_collision() {
    let cap0 = Capsule::new(Vec2::new(2.0, 0.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(-1.0, 2.0), 2.0);

    let tri0 = Triangle::new(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, -1.0),
    ]);
    let tri1 = Triangle::new(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(-1.0, 0.0),
    ]);

    test_collides(
        &cap0,
        Vec2::new(0.0, 1.0),
        &tri1,
        Vec2::new(-3.9, 1.0),
        0.1,
        Vec2::new(1.0, 0.0),
    );
    test_collides(
        &cap1,
        Vec2::new(-3.0, -2.0),
        &tri0,
        Vec2::new(-4.0, 2.9),
        0.1,
        Vec2::new(0.0, -1.0),
    );

    test_does_not_collide(&cap1, Vec2::zero(), &tri1, Vec2::new(2.5, -4.5));
    test_does_not_collide(&cap0, Vec2::new(10.0, 15.0), &tri0, Vec2::zero());
}
