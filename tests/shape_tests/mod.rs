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
    assert!(l.collides(l_pos, r, r_pos));
    assert!(r.collides(r_pos, l, l_pos));

    assert!(l.sat_collides(l_pos, r, r_pos));
    assert!(r.sat_collides(r_pos, l, l_pos));

    let l_res = l.collision_resolution(l_pos, r, r_pos);
    assert!(l_res.colliding);
    assert!(l_res.penetration.is_difference_trivial(expected_pen));
    assert!(l_res.axis.rotate_counter_90().perp(expected_axis));
    assert!(l_res.axis.dot(expected_axis).is_sign_positive());

    let r_res = r.collision_resolution(r_pos, l, l_pos);
    assert!(r_res.colliding);
    assert!(r_res.penetration.is_difference_trivial(expected_pen));
    assert!(r_res.axis.rotate_counter_90().perp(expected_axis));
    assert!(r_res.axis.dot(expected_axis).is_sign_negative());

    let l_res_sat = l.sat_collision_resolution(l_pos, r, r_pos);
    assert!(l_res_sat.colliding);
    assert!(l_res_sat.penetration.is_difference_trivial(expected_pen));
    assert!(l_res_sat.axis.rotate_counter_90().perp(expected_axis));
    assert!(l_res_sat.axis.dot(expected_axis).is_sign_positive());

    let r_res_sat = r.sat_collision_resolution(r_pos, l, l_pos);
    assert!(r_res_sat.colliding);
    assert!(r_res_sat.penetration.is_difference_trivial(expected_pen));
    assert!(r_res_sat.axis.rotate_counter_90().perp(expected_axis));
    assert!(r_res_sat.axis.dot(expected_axis).is_sign_negative());
}

pub fn test_does_not_collide<T: NumTolerance>(
    l: &(impl Shapeable<T> + SATable<T>),
    l_pos: Vec2<T>,
    r: &(impl Shapeable<T> + SATable<T>),
    r_pos: Vec2<T>,
) {
    assert!(!l.collides(l_pos, r, r_pos));
    assert!(!r.collides(r_pos, l, l_pos));

    assert!(!l.sat_collides(l_pos, r, r_pos));
    assert!(!r.sat_collides(r_pos, l, l_pos));

    let l_res = l.collision_resolution(l_pos, r, r_pos);
    assert!(!l_res.colliding);

    let r_res = r.collision_resolution(r_pos, l, l_pos);
    assert!(!r_res.colliding);

    let l_res_sat = l.sat_collision_resolution(l_pos, r, r_pos);
    assert!(!l_res_sat.colliding);

    let r_res_sat = r.sat_collision_resolution(r_pos, l, l_pos);
    assert!(!r_res_sat.colliding);
}
