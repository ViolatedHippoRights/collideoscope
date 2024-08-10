use collideoscope::{
    narrow::shapes::{
        aabb::AABB, capsule::Capsule, circle::Circle, p_gram::Pgram, polygon::Polygon,
        triangle::Triangle,
    },
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{test_collides, test_does_not_collide};

#[test]
fn test_polygon_aabb_collision() {
    let box0 = AABB::new(2.0, 2.0);

    let poly0 = Polygon::new(vec![
        Vec2::new(0.0, 1.0),
        Vec2::new(-0.5, 0.5),
        Vec2::new(-0.25, -0.5),
        Vec2::new(0.25, -0.5),
        Vec2::new(0.5, 0.5),
    ])
    .unwrap();
    let poly1 = Polygon::new(vec![Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::zero()]).unwrap();

    test_collides(
        &poly0,
        Vec2::zero(),
        &box0,
        Vec2::new(0.0, 1.5),
        0.5,
        Vec2::new(0.0, -1.0),
    );
    test_collides(
        &poly1,
        Vec2::new(2.0, 1.4),
        &box0,
        Vec2::new(3.9, 1.0),
        0.1,
        Vec2::new(-1.0, 0.0),
    );

    test_does_not_collide(&poly1, Vec2::new(1.0, -1.0), &box0, Vec2::new(0.0, 1.0));
    test_does_not_collide(&poly0, Vec2::new(10.0, 5.7), &box0, Vec2::zero());
}

#[test]
fn test_polygon_capsule_collision() {
    let cap0 = Capsule::new(Vec2::new(1.0, 0.0), 1.0);
    let cap1 = Capsule::new(Vec2::new(1.0, -1.0), 2.0);

    let poly0 = Polygon::new(vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.5, 1.0),
        Vec2::new(-0.5, 1.0),
        Vec2::new(-1.0, 0.0),
    ])
    .unwrap();
    let poly1 = Polygon::new(vec![
        Vec2::zero(),
        Vec2::new(1.0, -1.0),
        Vec2::new(-1.0, -0.5),
    ])
    .unwrap();

    test_collides(
        &poly0,
        Vec2::new(2.8, 0.0),
        &cap0,
        Vec2::zero(),
        0.2,
        Vec2::new(1.0, 0.0),
    );
    test_collides(
        &poly1,
        Vec2::zero(),
        &cap1,
        Vec2::new(-1.0, 2.9),
        0.1,
        Vec2::new(0.0, -1.0),
    );

    test_does_not_collide(&poly0, Vec2::new(-2.0, -2.0), &cap1, Vec2::new(1.0, 1.0));
    test_does_not_collide(&poly1, Vec2::new(10.0, 15.5), &cap0, Vec2::new(-35.4, 1.0));
}

#[test]
fn test_polygon_circle_collision() {
    let circ = Circle::new(2.4);

    let poly0 = Polygon::new(vec![
        Vec2::new(-1.0, 1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(1.0, -3.0),
        Vec2::new(-1.0, -1.0),
    ])
    .unwrap();
    let poly1 = Polygon::new(vec![
        Vec2::zero(),
        Vec2::new(2.0, 0.0),
        Vec2::new(-1.0, 1.0),
    ])
    .unwrap();

    test_collides(
        &poly0,
        Vec2::zero(),
        &circ,
        Vec2::new(1.0, -4.4),
        1.0,
        Vec2::new(0.0, 1.0),
    );
    test_collides(
        &poly1,
        Vec2::new(3.0, -2.5),
        &circ,
        Vec2::new(0.4, -1.5),
        0.8,
        Vec2::new(1.0, 0.0),
    );

    test_does_not_collide(&poly0, Vec2::new(1.0, 4.4), &circ, Vec2::zero());
    test_does_not_collide(&poly1, Vec2::zero(), &circ, Vec2::new(10000.0, 0.0));
}

#[test]
fn test_polygon_pgram_collision() {
    let gram0 = Pgram::new(Vec2::new(3.0, 0.0), Vec2::new(0.0, 2.0));
    let gram1 = Pgram::new(Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0));

    let poly0 = Polygon::new(vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(-1.0, -0.5),
    ])
    .unwrap();
    let poly1 = Polygon::new(vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
        Vec2::zero(),
    ])
    .unwrap();

    test_collides(
        &poly0,
        Vec2::zero(),
        &gram0,
        Vec2::new(0.0, 1.5),
        0.5,
        Vec2::new(0.0, -1.0),
    );
    test_collides(
        &poly1,
        Vec2::new(2.1, -0.1),
        &gram1,
        Vec2::new(1.3, 0.4),
        0.2,
        Vec2::new(1.0, 0.0),
    );

    test_does_not_collide(&poly0, Vec2::new(1.6, 1.1), &gram1, Vec2::zero());
    test_does_not_collide(&poly1, Vec2::new(95.6, 10.0), &gram0, Vec2::new(1.0, -2.0));
}

#[test]
fn test_polygon_polygon_collision() {
    let poly0 = Polygon::new(vec![
        Vec2::new(1.0, -1.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(-2.0, 0.0),
        Vec2::new(-1.0, -1.0),
    ])
    .unwrap();
    let poly1 = Polygon::new(vec![
        Vec2::new(0.5, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(-1.0, 1.0),
        Vec2::new(-0.5, 0.0),
    ])
    .unwrap();
    let poly2 = Polygon::new(vec![Vec2::zero(), Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)]).unwrap();

    test_collides(
        &poly0,
        Vec2::new(0.5, -0.8),
        &poly2,
        Vec2::zero(),
        0.2,
        Vec2::new(0.0, -1.0),
    );
    test_collides(
        &poly2,
        Vec2::new(3.0, 1.0),
        &poly1,
        Vec2::new(3.25, 1.7),
        0.3,
        Vec2::new(0.0, -1.0),
    );

    test_does_not_collide(&poly1, Vec2::zero(), &poly1, Vec2::new(1.1, -1.0));
    test_does_not_collide(&poly2, Vec2::new(10.0, -5.0), &poly0, Vec2::new(-4.7, 1.0));
}

#[test]
fn test_polygon_triangle_collision() {
    let tri0 = Triangle::new(&[Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0), Vec2::zero()]);
    let tri1 = Triangle::new(&[
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, -1.0),
        Vec2::new(-1.0, 0.5),
    ]);

    let poly0 = Polygon::new(vec![
        Vec2::new(-1.0, 1.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(1.0, -1.0),
        Vec2::new(-1.0, -1.0),
    ])
    .unwrap();
    let poly1 = Polygon::new(vec![
        Vec2::zero(),
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, 1.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(0.0, 2.0),
    ])
    .unwrap();

    test_collides(
        &poly0,
        Vec2::zero(),
        &tri0,
        Vec2::new(-1.5, 0.0),
        0.5,
        Vec2::new(1.0, 0.0),
    );
    test_collides(
        &poly1,
        Vec2::new(-1.0, -5.0),
        &tri1,
        Vec2::new(0.5, -5.4),
        0.1,
        Vec2::new(0.0, 1.0),
    );

    test_does_not_collide(&poly0, Vec2::new(1.0, -2.0), &tri1, Vec2::new(-0.6, -3.6));
    test_does_not_collide(&poly1, Vec2::new(10.0, 5.7), &tri0, Vec2::zero());
}
