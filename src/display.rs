use macroquad::prelude::*;

pub const REFERENCE_WIDTH: f32 = 800.0;
pub const REFERENCE_HEIGHT: f32 = 600.0;

#[derive(Debug, Clone, Copy)]
pub struct DisplayContext {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
}

impl DisplayContext {
    pub fn new() -> Self {
        let width = screen_width();
        let height = screen_height();

        Self {
            width,
            height,
            scale: (width / REFERENCE_WIDTH).min(height / REFERENCE_HEIGHT),
        }
    }

    pub fn refresh(&mut self) {
        self.width = screen_width();
        self.height = screen_height();

        self.scale = (self.width / REFERENCE_WIDTH).min(self.height / REFERENCE_HEIGHT);
    }

    pub fn x(&self, reference_x: f32) -> f32 {
        reference_x * self.width / REFERENCE_WIDTH
    }

    pub fn y(&self, reference_y: f32) -> f32 {
        reference_y * self.height / REFERENCE_HEIGHT
    }
}
