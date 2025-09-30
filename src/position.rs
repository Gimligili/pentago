use macroquad::math::{vec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowContext {
    width: f32,
    heigth: f32,
}

impl WindowContext {
    pub fn new(width: f32, height: f32) -> Self {
        WindowContext{
            width: width,
            heigth: height
        }
    }

    pub fn pos_from_middle(self, x: f32, y: f32, w: f32, h: f32) -> Vec2 {
        vec2(self.width  * ( x - w / 2.0 ), self.heigth * ( y - h / 2.0 ))
    }

    pub fn gen_size(self, w: f32, h: f32) -> Vec2 {
        vec2(w * self.width, h * self.heigth)
    }
}