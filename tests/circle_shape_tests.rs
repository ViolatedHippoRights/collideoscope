use float_eq::assert_float_eq;
use num::Float;

use collideoscope::{
    narrow::{sat::SATable, shapes::circle::Circle},
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{dual_collides_test, dual_collision_test};

#[test]
fn test_circle_aabb_collision() {}

#[test]
fn test_circle_capsule_collision() {}

#[test]
fn test_circle_circle_collision() {
    let circ0 = Circle::new(5.5);
    let circ1 = Circle::new(1.0);

    let res0 = circ0.collision_resolution(Vec2::zero(), &circ1, Vec2::new(6.0, 0.0));
    assert!(res0.colliding);
    assert_float_eq!(res0.penetration, 0.5, abs <= 0.01);
    assert!(res0.axis.perp(Vec2::new(0.0, 1.0)));
    assert!(res0.axis.x.is_sign_negative());
    dual_collision_test(&circ0, Vec2::zero(), &circ1, Vec2::new(6.0, 0.0));

    let res1 = circ1.collision_resolution(
        Vec2::new(2.0, 1.0),
        &circ0,
        Vec2::new(2.0 + 6.0 / f32::sqrt(2.0), 1.0 + 6.0 / f32::sqrt(2.0)),
    );
    assert!(res1.colliding);
    assert_float_eq!(res1.penetration, 0.5, abs <= 0.01);
    assert!(res1.axis.perp(Vec2::new(-1.0, 1.0)));
    assert!(res1.axis.x.is_sign_negative());
    dual_collision_test(
        &circ1,
        Vec2::new(2.0, 1.0),
        &circ0,
        Vec2::new(2.0 + 6.0 / f32::sqrt(2.0), 1.0 + 6.0 / f32::sqrt(2.0)),
    );

    let res2 = circ1.collides(
        Vec2::new(2.0, 1.0),
        &circ1,
        Vec2::new(2.0 + 6.0 / f32::sqrt(2.0), 1.0 + 6.0 / f32::sqrt(2.0)),
    );
    assert!(!res2);
    dual_collides_test(
        &circ1,
        Vec2::new(2.0, 1.0),
        &circ1,
        Vec2::new(2.0 + 6.0 / f32::sqrt(2.0), 1.0 + 6.0 / f32::sqrt(2.0)),
    );
}

#[test]
fn test_circle_pgram_collision() {}

#[test]
fn test_circle_polygon_collision() {}

#[test]
fn test_circle_triangle_collision() {}
