use collideoscope::{
    narrow::{sat::SATable, shapes::Shapeable},
    vec2::Vec2,
    NumTolerance,
};

pub fn test_collides<T: NumTolerance>(
    l: &(impl Shapeable<T> + SATable<T>),
    l_pos: Vec2<T>,
    r: &(impl Shapeable<T> + SATable<T>),
    r_pos: Vec2<T>,
    expected_pen: T,
    expected_axis: Vec2<T>,
) {
    assert!(
        l.collides(l_pos, r, r_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );
    assert!(
        r.collides(r_pos, l, l_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );

    assert!(
        l.sat_collides(l_pos, r, r_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );
    assert!(
        r.sat_collides(r_pos, l, l_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );

    let l_res = l.collision_resolution(l_pos, r, r_pos);
    assert!(
        l_res.colliding,
        "Left: {}, {} | Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );
    assert!(
        l_res.penetration.is_difference_trivial(expected_pen),
        "Expected: {} | Actual: {}, Axis: {} {}",
        expected_pen,
        l_res.penetration,
        l_res.axis.x,
        l_res.axis.y
    );
    assert!(
        l_res.axis.rotate_counter_90().perp(expected_axis),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        l_res.axis.x,
        l_res.axis.y
    );
    assert!(
        l_res.axis.dot(expected_axis).is_sign_positive(),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        l_res.axis.x,
        l_res.axis.y
    );

    let r_res = r.collision_resolution(r_pos, l, l_pos);
    assert!(
        r_res.colliding,
        "Left: {}, {} - Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );
    assert!(
        r_res.penetration.is_difference_trivial(expected_pen),
        "Expected: {} | Actual: {}, Axis: {}, {}",
        expected_pen,
        r_res.penetration,
        r_res.axis.x,
        r_res.axis.y
    );
    assert!(
        r_res.axis.rotate_counter_90().perp(expected_axis),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        r_res.axis.x,
        r_res.axis.y
    );
    assert!(
        r_res.axis.dot(expected_axis).is_sign_negative(),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        r_res.axis.x,
        r_res.axis.y
    );

    let l_res_sat = l.sat_collision_resolution(l_pos, r, r_pos);
    assert!(
        l_res_sat.colliding,
        "Left: {}, {} | Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );
    assert!(
        l_res_sat.penetration.is_difference_trivial(expected_pen),
        "Expected: {} | Actual: {}",
        expected_pen,
        l_res_sat.penetration
    );
    assert!(
        l_res_sat.axis.rotate_counter_90().perp(expected_axis),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        l_res_sat.axis.x,
        l_res_sat.axis.y
    );
    assert!(
        l_res_sat.axis.dot(expected_axis).is_sign_positive(),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        l_res_sat.axis.x,
        l_res_sat.axis.y
    );

    let r_res_sat = r.sat_collision_resolution(r_pos, l, l_pos);
    assert!(
        r_res_sat.colliding,
        "Left: {}, {} | Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );
    assert!(
        r_res_sat.penetration.is_difference_trivial(expected_pen),
        "Expected: {} | Actual: {}",
        expected_pen,
        r_res_sat.penetration
    );
    assert!(
        r_res_sat.axis.rotate_counter_90().perp(expected_axis),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        r_res_sat.axis.x,
        r_res_sat.axis.y
    );
    assert!(
        r_res_sat.axis.dot(expected_axis).is_sign_negative(),
        "Expected: {}, {} | Actual: {}, {}",
        expected_axis.x,
        expected_axis.y,
        r_res_sat.axis.x,
        r_res_sat.axis.y
    );
}

pub fn test_does_not_collide<T: NumTolerance>(
    l: &(impl Shapeable<T> + SATable<T>),
    l_pos: Vec2<T>,
    r: &(impl Shapeable<T> + SATable<T>),
    r_pos: Vec2<T>,
) {
    assert!(
        !l.collides(l_pos, r, r_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );
    assert!(
        !r.collides(r_pos, l, l_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );

    assert!(
        !l.sat_collides(l_pos, r, r_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );
    assert!(
        !r.sat_collides(r_pos, l, l_pos),
        "Left: {}, {} | Right: {}, {}",
        l_pos.x,
        l_pos.y,
        r_pos.x,
        r_pos.y
    );

    let l_res = l.collision_resolution(l_pos, r, r_pos);
    assert!(
        !l_res.colliding,
        "Left: {}, {} | Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );

    let r_res = r.collision_resolution(r_pos, l, l_pos);
    assert!(
        !r_res.colliding,
        "Left: {}, {} | Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );

    let l_res_sat = l.sat_collision_resolution(l_pos, r, r_pos);
    assert!(
        !l_res_sat.colliding,
        "Left: {}, {} | Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );

    let r_res_sat = r.sat_collision_resolution(r_pos, l, l_pos);
    assert!(
        !r_res_sat.colliding,
        "Left: {}, {} | Right: {}, {}",
        l_pos.x, l_pos.y, r_pos.x, r_pos.y
    );
}
