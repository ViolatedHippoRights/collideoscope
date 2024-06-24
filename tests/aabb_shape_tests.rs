use collideoscope::{
    narrow::shapes::{
        aabb::AABB, capsule::Capsule, circle::Circle, p_gram::Pgram, triangle::Triangle,
    },
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{test_collides, test_does_not_collide};

#[test]
fn test_aabb_aabb_collision() {
    let box0 = AABB::new(10.0, 10.0);
    let box1 = AABB::new(2.0, 4.0);
    let box2 = AABB::new(4.0, 2.0);

    test_collides(
        &box0,
        Vec2::new(5.0, 5.0),
        &box1,
        Vec2::new(1.0, -1.5),
        0.5,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &box1,
        Vec2::zero(),
        &box2,
        Vec2::new(2.75, 0.5),
        0.25,
        Vec2::new(-1.0, 0.0),
    );
    test_collides(
        &box1,
        Vec2::zero(),
        &box2,
        Vec2::new(1.0, 1.9),
        1.1,
        Vec2::new(0.0, -1.0),
    );

    test_does_not_collide(&box2, Vec2::zero(), &box2, Vec2::new(8.75, 0.5));
    test_does_not_collide(&box0, Vec2::new(-1.0, 1.0), &box2, Vec2::new(15.0, 3.0));
}

#[test]
fn test_aabb_capsule_collision() {
    let aabb = AABB::new(3.0, 2.0);
    let cap = Capsule::new(Vec2::new(0.0, -2.0), 1.0);

    test_collides(
        &aabb,
        Vec2::new(0.6, 1.0),
        &cap,
        Vec2::new(3.0, 1.0),
        0.1,
        Vec2::new(-1.0, 0.0),
    );
    test_collides(
        &aabb,
        Vec2::new(1.98, -3.64),
        &cap,
        Vec2::zero(),
        0.2,
        Vec2::new(0.6, -0.8),
    );

    test_does_not_collide(&aabb, Vec2::zero(), &cap, Vec2::new(2.3, 3.8));
    test_does_not_collide(&aabb, Vec2::new(1.0, -5.0), &cap, Vec2::new(32.0, -10.0));
}

#[test]
fn test_aabb_circle_collision() {
    let aabb = AABB::new(2.0, 4.0);
    let circ = Circle::new(1.0);

    test_collides(
        &aabb,
        Vec2::new(1.0, 2.0),
        &circ,
        Vec2::new(0.5, 0.4),
        1.4,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &aabb,
        Vec2::zero(),
        &circ,
        Vec2::new(1.5, -2.5),
        1.0 - f32::sqrt(0.5),
        Vec2::new(-1.0, 1.0),
    );

    test_does_not_collide(&aabb, Vec2::new(2.0, 0.0), &circ, Vec2::new(0.25, -2.75));
    test_does_not_collide(&aabb, Vec2::zero(), &circ, Vec2::new(10.0, 5.0));
}

#[test]
fn test_aabb_pgram_collision() {
    let box0 = AABB::new(2.0, 2.0);
    let box1 = AABB::new(4.0, 2.0);

    let gram0 = Pgram::new(Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0));
    let gram1 = Pgram::new(Vec2::new(2.0, 1.0), Vec2::new(-1.0, -1.0));

    test_collides(
        &box0,
        Vec2::zero(),
        &gram0,
        Vec2::new(0.0, 1.9),
        0.1,
        Vec2::new(0.0, -1.0),
    );
    test_collides(
        &box1,
        Vec2::new(2.0, 3.0),
        &gram1,
        Vec2::new(-1.4, 2.0),
        0.1,
        Vec2::new(1.0, 0.0),
    );

    test_does_not_collide(&box0, Vec2::new(4.0, -1.0), &gram1, Vec2::new(0.6, -0.6));
    test_does_not_collide(&box1, Vec2::zero(), &gram0, Vec2::new(32.0, 5.0));
}

#[test]
fn test_aabb_polygon_collision() {}

#[test]
fn test_aabb_triangle_collision() {
    let aabb = AABB::new(1.0, 2.0);
    let tri0 = Triangle::new(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, -1.0),
    ]);
    let tri1 = Triangle::new(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, -1.0),
        Vec2::new(-2.0, 0.0),
    ]);

    test_collides(
        &aabb,
        Vec2::new(2.0, 1.0),
        &tri0,
        Vec2::new(2.0, 2.6),
        0.4,
        Vec2::new(0.0, -1.0),
    );
    test_collides(
        &aabb,
        Vec2::new(0.5, 1.0),
        &tri0,
        Vec2::new(-0.4, 2.4),
        f32::sqrt(0.02),
        Vec2::new(1.0, -1.0),
    );

    test_does_not_collide(&aabb, Vec2::new(10.0, 5.0), &tri1, Vec2::new(12.6, 5.5));
    test_does_not_collide(&aabb, Vec2::new(25.0, 4.17), &tri0, Vec2::zero());
}
