use collideoscope::{
    narrow::shapes::{aabb::AABB, capsule::Capsule, circle::Circle, triangle::Triangle},
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{test_collides, test_does_not_collide};

#[test]
fn test_triangle_aabb_collision() {
    let tri0 = Triangle::new(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(0.0, 3.0),
    ]);
    let tri1 = Triangle::new(&[
        Vec2::new(0.5, -1.0 / f64::sqrt(12.0)),
        Vec2::new(-0.5, -1.0 / f64::sqrt(12.0)),
        Vec2::new(0.0, 1.0 / f64::sqrt(3.0)),
    ]);

    let aabb0 = AABB::new(2.0, 4.0);
    let aabb1 = AABB::new(6.0, 1.0);

    test_collides(
        &tri0,
        Vec2::zero(),
        &aabb0,
        Vec2::new(2.7, 1.0),
        0.3,
        Vec2::new(-1.0, 0.0),
    );
    test_collides(
        &tri1,
        Vec2::new(1.0, 2.0),
        &aabb1,
        Vec2::new(-2.3, 2.5),
        1.0 / (20.0 * f64::sqrt(3.0)),
        Vec2::new(f64::sqrt(3.0), -1.0),
    );

    test_does_not_collide(&tri0, Vec2::new(1.0, 1.0), &aabb1, Vec2::new(5.0, 3.1));
    test_does_not_collide(&tri1, Vec2::zero(), &aabb0, Vec2::new(100.0, -0.1));
}

#[test]
fn test_triangle_capsule_collision() {
    let cap0 = Capsule::new(Vec2::new(5.0, 0.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(2.0, 2.0), 2.0);

    let tri = Triangle::new(&[
        Vec2::new(0.5, 0.0),
        Vec2::new(-0.5, 0.0),
        Vec2::new(0.0, -1.0),
    ]);

    test_collides(
        &tri,
        Vec2::zero(),
        &cap0,
        Vec2::new(1.0, -1.75),
        0.25,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &tri,
        Vec2::new(13.7, 8.0),
        &cap1,
        Vec2::new(9.6, 6.0),
        0.4,
        Vec2::new(1.0, 0.0),
    );

    test_does_not_collide(&tri, Vec2::new(6.6, 2.6), &cap0, Vec2::zero());
    test_does_not_collide(&tri, Vec2::zero(), &cap1, Vec2::new(13.5, 22.2));
}

#[test]
fn test_triangle_circle_collision() {
    let tri0 = Triangle::new(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ]);
    let tri1 = Triangle::new(&[
        Vec2::new(-1.0, -1.0 / f64::sqrt(12.0)),
        Vec2::new(1.0, -1.0 / f64::sqrt(12.0)),
        Vec2::new(0.0, 1.0 / f64::sqrt(3.0)),
    ]);
    let circ = Circle::new(2.0);

    test_collides(
        &tri0,
        Vec2::zero(),
        &circ,
        Vec2::new(1.9, 1.9),
        2.0 - 1.4 * f64::sqrt(2.0),
        Vec2::new(-1.0, -1.0),
    );
    test_collides(
        &tri1,
        Vec2::new(-1.0, 2.0),
        &circ,
        Vec2::new(-1.0, 3.9 + 1.0 / f64::sqrt(3.0)),
        0.1,
        Vec2::new(0.0, -1.0),
    );

    test_does_not_collide(&tri0, Vec2::zero(), &circ, Vec2::new(-1.8, 2.8));
    test_does_not_collide(&tri1, Vec2::new(100.0, 15.1), &circ, Vec2::new(32.1, -4.0));
}

#[test]
fn test_triangle_pgram_collision() {}

#[test]
fn test_triangle_polygon_collision() {}

#[test]
fn test_triangle_triangle_collision() {
    let tri0 = Triangle::new(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
    ]);
    let tri1 = Triangle::new(&[
        Vec2::new(0.5, -1.0 / f64::sqrt(12.0)),
        Vec2::new(-0.5, -1.0 / f64::sqrt(12.0)),
        Vec2::new(0.0, 1.0 / f64::sqrt(3.0)),
    ]);

    let root2 = 1.0 / f64::sqrt(2.0);

    test_collides(
        &tri0,
        Vec2::new(-0.5, 0.0),
        &tri1,
        Vec2::new(0.0, -0.5),
        (1.0 / f64::sqrt(3.0)) - 0.5,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &tri1,
        Vec2::new(0.75, 0.75),
        &tri0,
        Vec2::zero(),
        root2 - Vec2::new(0.25, 0.75 - 1.0 / f64::sqrt(12.0)).dot(Vec2::new(root2, root2)),
        Vec2::new(1.0, 1.0),
    );

    test_does_not_collide(&tri1, Vec2::new(2.0, 2.0), &tri1, Vec2::new(0.5, 0.5));
    test_does_not_collide(&tri0, Vec2::new(100.0, 1.0), &tri1, Vec2::new(75.0, 10.0));
}
