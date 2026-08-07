use macroquad::prelude::*;

use crate::display::DisplayContext;

pub struct Button<'a> {
    pub rect: Rect,
    pub text: &'a str,
    hover_t: f32,
}

impl<'a> Button<'a> {
    pub fn new(rect: Rect, text: &'a str) -> Self {
        Self {
            rect,
            text,
            hover_t: 0.0,
        }
    }

    pub fn is_hovered(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();
        self.rect.contains(vec2(mouse_x, mouse_y))
    }

    pub fn is_clicked(&self) -> bool {
        self.is_hovered() && is_mouse_button_pressed(MouseButton::Left)
    }

    pub fn update(&mut self) {
        let target = if self.is_hovered() { 1.0 } else { 0.0 };

        let speed = 8.0;

        self.hover_t += (target - self.hover_t) * speed * get_frame_time();

        self.hover_t = self.hover_t.clamp(0.0, 1.0);
    }

    pub fn draw(&self, font: &Font, display: &DisplayContext) {
        let hovered = self.is_hovered();

        let base_color = Color::from_rgba(35, 30, 28, 255);
        let hover_color = Color::from_rgba(60, 48, 40, 255);

        let background = if hovered { hover_color } else { base_color };

        let hover_scale = 1.0 + self.hover_t * 0.05;

        let width = self.rect.w * hover_scale;

        let height = self.rect.h * hover_scale;

        let x = self.rect.x - (width - self.rect.w) / 2.0;

        let y = self.rect.y - (height - self.rect.h) / 2.0;

        draw_rectangle(x, y, width, height, background);

        let border_width = 2.0 * display.scale;

        draw_rectangle_lines(x, y, width, height, border_width, GOLD);

        let font_size = (28.0 * display.scale) as u16;

        let text_size = measure_text(self.text, Some(font), font_size, 1.0);

        let text_x = self.rect.x + (self.rect.w - text_size.width) / 2.0;

        let text_y = self.rect.y + (self.rect.h + text_size.height) / 2.0 - 4.0 * display.scale;

        draw_text_ex(
            self.text,
            text_x,
            text_y,
            TextParams {
                font: Some(font),
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
