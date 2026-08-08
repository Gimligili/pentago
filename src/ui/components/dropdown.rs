use macroquad::prelude::*;

use crate::display::DisplayContext;

#[derive(Debug, Clone, Copy)]
pub struct DropdownState {
    pub open: bool,
}

impl DropdownState {
    pub fn new() -> Self {
        Self { open: false }
    }
}

impl Default for DropdownState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Dropdown<'a, T>
where
    T: Copy,
{
    pub rect: Rect,
    pub options: &'a [T],
    pub selected_index: &'a mut usize,
}

impl<'a, T> Dropdown<'a, T>
where
    T: Copy,
{
    pub fn new(rect: Rect, options: &'a [T], selected_index: &'a mut usize) -> Self {
        Self {
            rect,
            options,
            selected_index,
        }
    }

    fn open_area(&self) -> Rect {
        Rect::new(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h * (self.options.len() as f32 + 1.0),
        )
    }

    pub fn draw<F>(
        &mut self,
        state: &mut DropdownState,
        font: &Font,
        display: &DisplayContext,
        label: F,
    ) where
        F: Fn(T) -> &'static str,
    {
        if self.options.is_empty() {
            return;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let mouse = vec2(mouse_x, mouse_y);

        let background = Color::from_rgba(35, 30, 28, 255);
        let hover_background = Color::from_rgba(60, 48, 40, 255);
        let font_size = (28.0 * display.scale) as u16;
        let border_width = 2.0 * display.scale;
        let padding = 15.0 * display.scale;
        let main_hovered = self.rect.contains(mouse);

        if state.open
            && is_mouse_button_pressed(MouseButton::Left)
            && !self.open_area().contains(mouse)
        {
            state.open = false;
            return;
        }

        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            if main_hovered {
                hover_background
            } else {
                background
            },
        );

        draw_rectangle_lines(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            border_width,
            GOLD,
        );

        let selected = self.options[*self.selected_index];

        let selected_label = label(selected);

        let text_size = measure_text(selected_label, Some(font), font_size, 1.0);

        draw_text_ex(
            selected_label,
            self.rect.x + padding,
            self.rect.y + (self.rect.h + text_size.height) / 2.0 - 4.0 * display.scale,
            TextParams {
                font: Some(font),
                font_size,
                color: WHITE,
                ..Default::default()
            },
        );

        // Arrow
        let arrow_x = self.rect.x + self.rect.w - 24.0 * display.scale;

        let arrow_y = self.rect.y + self.rect.h / 2.0;

        let arrow_size = 6.0 * display.scale;

        if state.open {
            draw_triangle(
                vec2(arrow_x - arrow_size, arrow_y + arrow_size / 2.0),
                vec2(arrow_x + arrow_size, arrow_y + arrow_size / 2.0),
                vec2(arrow_x, arrow_y - arrow_size),
                WHITE,
            );
        } else {
            draw_triangle(
                vec2(arrow_x - arrow_size, arrow_y - arrow_size / 2.0),
                vec2(arrow_x + arrow_size, arrow_y - arrow_size / 2.0),
                vec2(arrow_x, arrow_y + arrow_size),
                WHITE,
            );
        }

        if main_hovered && is_mouse_button_pressed(MouseButton::Left) {
            state.open = !state.open;
            return;
        }

        if !state.open {
            return;
        }

        for (index, option) in self.options.iter().copied().enumerate() {
            let option_rect = Rect::new(
                self.rect.x,
                self.rect.y + self.rect.h * (index as f32 + 1.0),
                self.rect.w,
                self.rect.h,
            );

            let hovered = option_rect.contains(mouse);

            draw_rectangle(
                option_rect.x,
                option_rect.y,
                option_rect.w,
                option_rect.h,
                if hovered {
                    hover_background
                } else {
                    background
                },
            );

            draw_rectangle_lines(
                option_rect.x,
                option_rect.y,
                option_rect.w,
                option_rect.h,
                border_width,
                GOLD,
            );

            let option_label = label(option);

            let option_text_size = measure_text(option_label, Some(font), font_size, 1.0);

            draw_text_ex(
                option_label,
                option_rect.x + padding,
                option_rect.y + (option_rect.h + option_text_size.height) / 2.0
                    - 4.0 * display.scale,
                TextParams {
                    font: Some(font),
                    font_size,
                    color: WHITE,
                    ..Default::default()
                },
            );

            if hovered && is_mouse_button_pressed(MouseButton::Left) {
                *self.selected_index = index;
                state.open = false;
                return;
            }
        }
    }
}
