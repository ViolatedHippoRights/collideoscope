use collideoscope::{
    narrow::shapes::{
        aabb::AABB, capsule::Capsule, circle::Circle, p_gram::Pgram, triangle::Triangle,
    },
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{test_collides, test_does_not_collide};

#[test]
fn test_pgram_aabb_collision() {
    let gram0 = Pgram::new(Vec2::new(1.0, 1.0), Vec2::new(3.0, 0.0));
    let gram1 = Pgram::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0));

    let box0 = AABB::new(2.0, 2.0);
    let box1 = AABB::new(2.0, 8.0);

    test_collides(
        &gram0,
        Vec2::zero(),
        &box0,
        Vec2::new(2.4, -0.9),
        f64::sqrt(0.02),
        Vec2::new(-1.0, 1.0),
    );
    test_collides(
        &gram1,
        Vec2::new(-2.0, 4.0),
        &box1,
        Vec2::new(-0.5, 4.0),
        0.5,
        Vec2::new(-1.0, 0.0),
    );

    test_does_not_collide(&gram0, Vec2::new(-2.6, 4.0), &box1, Vec2::zero());
    test_does_not_collide(&gram1, Vec2::new(32.8, 10.0), &box0, Vec2::new(15.1, 23.2));
}

#[test]
fn test_pgram_capsule_collision() {
    let gram0 = Pgram::new(Vec2::new(0.0, 2.0), Vec2::new(-1.0, -1.0));
    let gram1 = Pgram::new(Vec2::new(2.0, 1.0), Vec2::new(2.0, -1.0));

    let cap0 = Capsule::new(Vec2::new(2.0, 0.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(1.0, -1.0), 2.0);

    test_collides(
        &gram0,
        Vec2::zero(),
        &cap0,
        Vec2::new(0.0, -2.0),
        0.5,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &gram1,
        Vec2::new(3.0, 2.0),
        &cap1,
        Vec2::new(7.8, 1.0),
        0.2,
        Vec2::new(-1.0, 0.0),
    );

    test_does_not_collide(&gram0, Vec2::new(-4.6, 4.0), &cap1, Vec2::zero());
    test_does_not_collide(&gram1, Vec2::new(56.7, -10.0), &cap0, Vec2::new(1.1, 80.0));
}

#[test]
fn test_pgram_circle_collision() {
    let gram0 = Pgram::new(Vec2::new(3.0, 0.0), Vec2::new(1.0, -1.0));
    let gram1 = Pgram::new(Vec2::new(1.0, 2.0), Vec2::new(1.0, -2.0));

    let circ = Circle::new(1.0);

    test_collides(
        &gram0,
        Vec2::zero(),
        &circ,
        Vec2::new(0.0, 1.2),
        0.3,
        Vec2::new(0.0, -1.0),
    );
    test_collides(
        &gram1,
        Vec2::new(1.0, 2.0),
        &circ,
        Vec2::new(2.4, 3.0),
        1.0 - (1.8 / f64::sqrt(5.0)),
        Vec2::new(-2.0, -1.0),
    );

    test_does_not_collide(&gram0, Vec2::new(-2.9, 0.0), &circ, Vec2::zero());
    test_does_not_collide(&gram1, Vec2::new(34.3, 10.0), &circ, Vec2::new(33.3, 56.2));
}

#[test]
fn test_pgram_pgram_collision() {
    let gram0 = Pgram::new(Vec2::new(2.0, 1.0), Vec2::new(0.0, 1.0));
    let gram1 = Pgram::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0));
    let gram2 = Pgram::new(Vec2::new(3.0, 0.0), Vec2::new(0.0, 2.0));

    test_collides(
        &gram0,
        Vec2::zero(),
        &gram2,
        Vec2::new(0.0, -1.75),
        0.25,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &gram1,
        Vec2::new(3.0, -4.0),
        &gram0,
        Vec2::new(4.8, -3.5),
        0.2,
        Vec2::new(-1.0, 0.0),
    );

    test_does_not_collide(&gram1, Vec2::new(2.0, 2.0), &gram2, Vec2::zero());
    test_does_not_collide(
        &gram2,
        Vec2::new(100.0, 0.0),
        &gram0,
        Vec2::new(55.7, -32.5),
    );
}

#[test]
fn test_pgram_polygon_collision() {}

#[test]
fn test_pgram_triangle_collision() {
    let gram0 = Pgram::new(Vec2::new(1.0, 1.0), Vec2::new(-2.0, -1.0));
    let gram1 = Pgram::new(Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0));

    let tri = Triangle::new(&[
        Vec2::new(0.0, 0.0),
        Vec2::new(3.0, 0.0),
        Vec2::new(0.0, 4.0),
    ]);

    test_collides(
        &gram0,
        Vec2::zero(),
        &tri,
        Vec2::new(1.4, 0.0),
        0.1,
        Vec2::new(-1.0, 0.0),
    );
    test_collides(
        &gram1,
        Vec2::new(-1.0, -1.5),
        &tri,
        Vec2::new(-2.0, -1.0),
        0.5,
        Vec2::new(0.0, -1.0),
    );

    test_does_not_collide(&gram0, Vec2::new(-0.5, 4.7), &tri, Vec2::zero());
    test_does_not_collide(&gram1, Vec2::new(12.0, 1.0), &tri, Vec2::new(-20.5, 13.5));
}
