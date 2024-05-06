use float_eq::assert_float_eq;
use num::Float;

use collideoscope::{
    narrow::{
        sat::SATable,
        shapes::{aabb::AABB, circle::Circle, triangle::Triangle},
    },
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{dual_collides_test, dual_collision_test};

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
    dual_collision_test(&box0, Vec2::new(5.0, 5.0), &box1, Vec2::new(1.0, -1.5));

    let res1 = box1.collision_resolution(Vec2::zero(), &box2, Vec2::new(2.75, 0.5));
    assert!(res1.colliding);
    assert_float_eq!(res1.penetration, 0.25, abs <= 0.01);
    assert!(res1.axis.perp(Vec2::new(0.0, 1.0)));
    assert!(res1.axis.x.is_sign_negative());
    dual_collision_test(&box1, Vec2::zero(), &box2, Vec2::new(2.75, 0.5));

    let res2 = box2.collision_resolution(Vec2::zero(), &box2, Vec2::new(8.75, 0.5));
    assert!(!res2.colliding);
    assert_float_eq!(res2.penetration, f32::max_value(), abs <= 0.01);
    assert_float_eq!(res2.axis.length_squared(), 0.0, abs <= 0.01);
    dual_collision_test(&box2, Vec2::zero(), &box2, Vec2::new(8.75, 0.5));

    assert!(!box0.collides(Vec2::zero(), &box2, Vec2::new(15.0, 3.0)));
    dual_collides_test(&box0, Vec2::zero(), &box2, Vec2::new(15.0, 3.0));

    assert!(box1.collides(Vec2::zero(), &box2, Vec2::new(1.0, 1.9)));
    dual_collides_test(&box1, Vec2::zero(), &box2, Vec2::new(1.0, 1.9));
}

#[test]
fn test_aabb_capsule_collision() {}

#[test]
fn test_aabb_circle_collision() {
    let aabb = AABB::new(2.0, 4.0);
    let circ = Circle::new(1.0);

    let res0 = aabb.collision_resolution(Vec2::new(1.0, 2.0), &circ, Vec2::new(0.5, 0.4));
    assert!(res0.colliding);
    assert_float_eq!(res0.penetration, 1.4, abs <= 0.01);
    assert_float_eq!(res0.axis.y, 1.0, abs <= 0.01);
    assert_float_eq!(res0.axis.x, 0.0, abs <= 0.01);
    dual_collision_test(&aabb, Vec2::new(1.0, 2.0), &circ, Vec2::new(0.5, 0.5));

    let res1 = aabb.collision_resolution(Vec2::new(0.0, 0.0), &circ, Vec2::new(1.5, -2.5));
    assert!(res1.colliding);
    assert_float_eq!(res1.penetration, 1.0 - f32::sqrt(0.5), abs <= 0.01);
    assert!(res1.axis.perp(Vec2::new(1.0, 1.0)));
    dual_collision_test(&aabb, Vec2::new(0.0, 0.0), &circ, Vec2::new(1.5, -0.5));

    assert!(aabb.collides(Vec2::zero(), &circ, Vec2::new(1.9, 0.0)));
    assert!(!aabb.collides(Vec2::new(2.0, 0.0), &circ, Vec2::new(0.25, -2.75)));
}

#[test]
fn test_aabb_pgram_collision() {}

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

    let res0 = aabb.collision_resolution(Vec2::new(2.0, 1.0), &tri0, Vec2::new(2.0, 2.5));
    assert!(res0.colliding);
    assert_float_eq!(res0.penetration, 0.5, abs <= 0.01);
    assert_float_eq!(res0.axis.y, -1.0, abs <= 0.01);
    assert_float_eq!(res0.axis.x, 0.0, abs <= 0.01);
    dual_collision_test(&aabb, Vec2::new(2.0, 1.0), &tri0, Vec2::new(3.0, 4.5));

    let res1 = aabb.collision_resolution(Vec2::new(0.5, 1.0), &tri0, Vec2::new(-0.4, 2.4));
    assert!(res1.colliding);
    assert_float_eq!(res1.penetration, f32::sqrt(0.02), abs <= 0.01);
    assert!(res1.axis.perp(Vec2::new(1.0, 1.0)));
    dual_collision_test(&aabb, Vec2::new(0.5, 1.0), &tri0, Vec2::new(-0.4, 2.4));

    assert!(aabb.collides(Vec2::new(10.0, 5.0), &tri1, Vec2::new(12.4, 5.5)));
    dual_collides_test(&aabb, Vec2::new(10.0, 5.0), &tri1, Vec2::new(12.4, 5.5));

    assert!(!aabb.collides(Vec2::new(10.0, 5.0), &tri1, Vec2::new(12.6, 5.5)));
    dual_collides_test(&aabb, Vec2::new(10.0, 5.0), &tri1, Vec2::new(12.6, 5.5));
}
