use macroquad::prelude::*;

use crate::display::DisplayContext;

const BOX_SIZE_REF: f32 = 28.0;
const LABEL_GAP_REF: f32 = 12.0;
const FONT_SIZE_REF: f32 = 24.0;
const BORDER_WIDTH_REF: f32 = 2.0;

const BASE_COLOR: Color = Color::from_rgba(35, 30, 28, 255);
const HOVER_COLOR: Color = Color::from_rgba(60, 48, 40, 255);

pub struct Checkbox<'a> {
    pub position: Vec2,
    pub label: &'a str,
    pub checked: bool,
    pub enabled: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(position: Vec2, label: &'a str, checked: bool) -> Self {
        Self {
            position,
            label,
            checked,
            enabled: true,
        }
    }

    fn rect(&self, display: &DisplayContext) -> Rect {
        let size = BOX_SIZE_REF * display.scale;

        Rect::new(self.position.x, self.position.y, size, size)
    }

    pub fn is_hovered(&self, display: &DisplayContext) -> bool {
        if !self.enabled {
            return false;
        }

        let (mouse_x, mouse_y) = mouse_position();

        self.rect(display).contains(vec2(mouse_x, mouse_y))
    }

    pub fn update(&mut self, display: &DisplayContext) {
        if self.is_hovered(display) && is_mouse_button_pressed(MouseButton::Left) {
            self.checked = !self.checked;
        }
    }

    pub fn draw(&self, font: &Font, display: &DisplayContext) {
        let rect = self.rect(display);

        let background = if self.is_hovered(display) {
            HOVER_COLOR
        } else {
            BASE_COLOR
        };

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, background);

        draw_rectangle_lines(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            BORDER_WIDTH_REF * display.scale,
            GOLD,
        );

        if self.checked {
            let margin = 6.0 * display.scale;

            draw_line(
                rect.x + margin,
                rect.y + rect.h / 2.0,
                rect.x + rect.w * 0.4,
                rect.y + rect.h - margin,
                3.0 * display.scale,
                WHITE,
            );

            draw_line(
                rect.x + rect.w * 0.35,
                rect.y + rect.h - margin,
                rect.x + rect.w - margin,
                rect.y + margin,
                3.0 * display.scale,
                WHITE,
            );
        }

        let font_size = (FONT_SIZE_REF * display.scale) as u16;

        let text_size = measure_text(self.label, Some(font), font_size, 1.0);

        draw_text_ex(
            self.label,
            rect.x + rect.w + LABEL_GAP_REF * display.scale,
            rect.y + (rect.h + text_size.height) / 2.0 - 3.0 * display.scale,
            TextParams {
                font: Some(font),
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
