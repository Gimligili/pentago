use macroquad::math::{vec2, Vec2};

pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 600.;


pub fn pos_from_middle(x: f32, y: f32, w: f32, h: f32) -> Vec2 {
    vec2(WIDTH  * ( x - w / 2.0 ), HEIGHT * ( y - h / 2.0 ))
}

pub fn gen_size(w: f32, h: f32) -> Vec2 {
    vec2(w * WIDTH, h * HEIGHT)
}