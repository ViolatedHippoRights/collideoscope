use float_eq::assert_float_eq;
use num::Float;

use collideoscope::{
    narrow::{sat::SATable, shapes::aabb::AABB},
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{test_collides, test_does_not_collide};

#[test]
fn test_capsule_aabb_collision() {}

#[test]
fn test_capsule_capsule_collision() {}

#[test]
fn test_capsule_circle_collision() {}

#[test]
fn test_capsule_pgram_collision() {}

#[test]
fn test_capsule_polygon_collision() {}

#[test]
fn test_capsule_triangle_collision() {}
