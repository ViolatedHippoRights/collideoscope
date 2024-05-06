use float_eq::assert_float_eq;
use num::Float;

use collideoscope::{
    narrow::{
        sat::SATable,
        shapes::{aabb::AABB, triangle::Triangle},
    },
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{dual_collides_test, dual_collision_test};

#[test]
fn test_triangle_aabb_collision() {}

#[test]
fn test_triangle_capsule_collision() {}

#[test]
fn test_triangle_circle_collision() {}

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

    let res0 = tri0.collision_resolution(Vec2::new(-0.5, 0.0), &tri1, Vec2::new(0.0, -0.5));
    assert!(res0.colliding);
    assert_float_eq!(res0.penetration, (1.0 / f64::sqrt(3.0)) - 0.5, abs <= 0.01);
    assert!(res0.axis.perp(Vec2::new(1.0, 0.0)));
    assert!(res0.axis.y.is_sign_positive());
    dual_collision_test(&tri0, Vec2::new(-0.5, 0.0), &tri1, Vec2::new(0.0, -0.5));

    let res1 = tri1.collision_resolution(Vec2::new(0.75, 0.75), &tri0, Vec2::zero());
    assert!(res1.colliding);
    assert_float_eq!(
        res1.penetration,
        (Vec2::new(f64::sqrt(3.0) * 0.25, 1.0 - f64::sqrt(3.0) * 0.25)
            - Vec2::new(0.25, 0.75 - 1.0 / f64::sqrt(12.0)))
        .length(),
        abs <= 0.01
    );
    assert!(res1.axis.perp(Vec2::new(1.0, -1.0)));
    assert!(res1.axis.y.is_sign_positive());
    dual_collision_test(&tri1, Vec2::new(0.75, 0.75), &tri0, Vec2::zero());

    let coll = tri1.collision_resolution(Vec2::new(2.0, 2.0), &tri1, Vec2::new(0.5, 0.5));
    assert!(!coll.colliding);
    assert!(!tri1.collides(Vec2::new(2.0, 2.0), &tri1, Vec2::new(0.5, 0.5)));
    dual_collides_test(&tri1, Vec2::new(2.0, 2.0), &tri0, Vec2::new(0.5, 0.5));
}
