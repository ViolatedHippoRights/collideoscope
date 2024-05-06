use float_eq::assert_float_eq;
use num::Float;

use collideoscope::{
    narrow::{sat::SATable, shapes::aabb::AABB},
    vec2::Vec2,
};

pub mod shape_tests;
use shape_tests::{dual_collides_test, dual_collision_test};

#[test]
fn test_pgram_aabb_collision() {}

#[test]
fn test_pgram_capsule_collision() {}

#[test]
fn test_pgram_circle_collision() {}

#[test]
fn test_pgram_pgram_collision() {}

#[test]
fn test_pgram_polygon_collision() {}

#[test]
fn test_pgram_triangle_collision() {}