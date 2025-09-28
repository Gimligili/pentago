use macroquad::math::{vec2, Vec2};

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;


pub fn middle_pos(x: f32, y: f32, w: f32, h: f32) -> Vec2 {
    vec2(x * WIDTH as f32  - (w / 2.0), y * HEIGHT as f32 - (h / 2.0))
}