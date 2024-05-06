use collideoscope::{
    narrow::{sat::SATable, shapes::Shapeable},
    vec2::Vec2,
    NumTolerance,
};

pub fn dual_collides_test<T: NumTolerance>(
    l: &impl SATable<T>,
    l_pos: Vec2<T>,
    r: &(impl Shapeable<T> + SATable<T>),
    r_pos: Vec2<T>,
) {
    let coll = l.collides(l_pos, r, r_pos);
    let coll_sat = l.sat_collides(l_pos, r, r_pos);

    assert!(coll == coll_sat);
}

pub fn dual_collision_test<T: NumTolerance>(
    l: &impl SATable<T>,
    l_pos: Vec2<T>,
    r: &(impl Shapeable<T> + SATable<T>),
    r_pos: Vec2<T>,
) {
    let res = l.collision_resolution(l_pos, r, r_pos);
    let res_sat = l.sat_collision_resolution(l_pos, r, r_pos);
    assert!(res.colliding == res_sat.colliding);
    assert!(res.penetration.is_difference_trivial(res_sat.penetration));
    assert!(res.axis.x.is_difference_trivial(res_sat.axis.x));
    assert!(res.axis.y.is_difference_trivial(res_sat.axis.y));
}
