use rand::{thread_rng, Rng};
use sfml::system::Vector2f;
use std::f32::consts::PI;
// CONSTS

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;


/// random true or false
pub fn random_bool() -> bool {
    let mut rng = thread_rng();
    let result: bool = rng.gen_bool(1.0 / 3.0);
    result
}

/// random number between min and max
pub fn random_number(min_v: f32, max_v: f32) -> f32 {
    let mut rng = thread_rng();

    let a = min_v.min(max_v);
    let b = min_v.max(max_v);

    let result: f32 = rng.gen_range(a, b);
    result
}

// degreese to radians
pub fn d_to_r(deg: f32) -> f32 {
    deg * PI / 180.0
}

/// get vector normal between two points
pub fn v2_normal(a: &Vector2f, b: &Vector2f) -> Vector2f {
    // (-y, x)
    Vector2f::new(-(b.y - a.y), b.x - a.x)
}

/// get the dot product of a vector
pub fn v2_dot(a: Vector2f, b: Vector2f) -> f32{
    let x = a.x * b.x;
    let y = a.y * b.y;
    x+y
}

#[allow(dead_code)]
/// get a vectors length
pub fn v2_length(v2f: Vector2f)->f32{
    let d = v2_dot(v2f,v2f);
    d.sqrt()
}

#[allow(dead_code)]
pub fn v2_length_sq(v2f: Vector2f)->f32{
    v2_dot(v2f,v2f)
}

#[allow(dead_code)]
/// get a unit vector
pub fn v2_unit(v2: Vector2f)->Vector2f{
    v2 * (1. / v2_length(v2))
}

#[allow(dead_code)]
/// get direction between to vectors
pub fn v2_direction(a: Vector2f, b: Vector2f) ->Vector2f{
    let c = a - b;
    v2_unit(c)
}

#[allow(dead_code)]
/// get distance srqt between two vectors
pub fn v2_distance_to(a: Vector2f, b: Vector2f)->f32{
    let c = a-b;
    v2_length(c)
}

#[allow(dead_code)]
/// get distance srqt between two vectors
pub fn v2_distance_to_sq(a: Vector2f, b: Vector2f)->f32{
    let c = a-b;
    v2_length_sq(c)
}

/// get angle between two vectors
pub fn v2_angle_to_point(a: Vector2f, b: Vector2f)->f32{
    // using sin-1
    // let dis = *a - *b;
    // let len = v2_length_sq(dis);
    // (dis.y / len).asin()

    let dis = a - b;
    dis.y.atan2(dis.x)
}
