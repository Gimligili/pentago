use macroquad::prelude::*;

use crate::display::{DisplayContext, Resolution, WindowMode};
use crate::ui::components::button::Button;
use crate::ui::components::checkbox::Checkbox;
use crate::ui::components::dropdown::{Dropdown, DropdownState};

const POPUP_WIDTH_REF: f32 = 500.0;
const POPUP_HEIGHT_REF: f32 = 360.0;

const DROPDOWN_WIDTH_REF: f32 = 300.0;
const DROPDOWN_HEIGHT_REF: f32 = 55.0;
const DROPDOWN_Y_REF: f32 = 145.0;

const BUTTON_WIDTH_REF: f32 = 180.0;
const BUTTON_HEIGHT_REF: f32 = 55.0;
const BUTTON_GAP_REF: f32 = 20.0;
const BUTTON_BOTTOM_MARGIN_REF: f32 = 30.0;

pub struct OptionsPopupState {
    pub open: bool,
    pub selected_resolution_index: usize,
    pub resolution_dropdown: DropdownState,
    pub fullscreen: bool,
}

fn resolution_index(display_resolution: Resolution) -> usize {
    Resolution::ALL
        .iter()
        .position(|resolution| *resolution == display_resolution)
        .unwrap_or(0)
}

impl OptionsPopupState {
    pub fn new(display: &DisplayContext) -> Self {
        let selected_resolution_index = resolution_index(display.resolution);
        Self {
            open: false,
            selected_resolution_index,
            resolution_dropdown: DropdownState::new(),
            fullscreen: display.window_mode == WindowMode::Fullscreen,
        }
    }

    pub fn open(&mut self, display: &DisplayContext) {
        self.open = true;
        self.selected_resolution_index = resolution_index(display.resolution);
        self.resolution_dropdown.open = false;
        self.fullscreen = display.window_mode == WindowMode::Fullscreen;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.resolution_dropdown.open = false;
    }
}

pub fn draw_options_popup(
    state: &mut OptionsPopupState,
    font: &Font,
    display: &mut DisplayContext,
) {
    if !state.open {
        return;
    }

    let scale = display.scale;

    let popup_width = POPUP_WIDTH_REF * scale;
    let popup_height = POPUP_HEIGHT_REF * scale;

    let popup = Rect::new(
        (display.width - popup_width) / 2.0,
        (display.height - popup_height) / 2.0,
        popup_width,
        popup_height,
    );

    // Dark overlay over the main menu
    draw_rectangle(
        0.0,
        0.0,
        display.width,
        display.height,
        Color::from_rgba(0, 0, 0, 150),
    );

    // Popup background
    draw_rectangle(
        popup.x,
        popup.y,
        popup.w,
        popup.h,
        Color::from_rgba(30, 25, 22, 255),
    );

    draw_rectangle_lines(popup.x, popup.y, popup.w, popup.h, 2.0 * scale, GOLD);

    // Title
    let title = "Options";
    let title_font_size = (42.0 * scale) as u16;

    let title_size = measure_text(title, Some(font), title_font_size, 1.0);

    draw_text_ex(
        title,
        popup.x + (popup.w - title_size.width) / 2.0,
        popup.y + 65.0 * scale,
        TextParams {
            font: Some(font),
            font_size: title_font_size,
            color: WHITE,
            ..Default::default()
        },
    );

    // Resolution label
    let label_font_size = (24.0 * scale) as u16;

    draw_text_ex(
        "Resolution",
        popup.x + 50.0 * scale,
        popup.y + 125.0 * scale,
        TextParams {
            font: Some(font),
            font_size: label_font_size,
            color: WHITE,
            ..Default::default()
        },
    );

    // Dropdown
    let dropdown_width = DROPDOWN_WIDTH_REF * scale;
    let dropdown_height = DROPDOWN_HEIGHT_REF * scale;

    let dropdown_rect = Rect::new(
        popup.x + (popup.w - dropdown_width) / 2.0,
        popup.y + DROPDOWN_Y_REF * scale,
        dropdown_width,
        dropdown_height,
    );

    let mut resolution_dropdown = Dropdown::new(
        dropdown_rect,
        &Resolution::ALL,
        &mut state.selected_resolution_index,
    );

    let mut fullscreen_checkbox = Checkbox::new(
        vec2(popup.x + 50.0 * scale, popup.y + 220.0 * scale),
        "Fullscreen",
        state.fullscreen,
    );

    fullscreen_checkbox.enabled = !state.resolution_dropdown.open;

    fullscreen_checkbox.update(display);
    fullscreen_checkbox.draw(font, display);

    state.fullscreen = fullscreen_checkbox.checked;

    // Buttons
    let button_width = BUTTON_WIDTH_REF * scale;
    let button_height = BUTTON_HEIGHT_REF * scale;
    let button_gap = BUTTON_GAP_REF * scale;
    let total_width = button_width * 2.0 + button_gap;

    let buttons_x = popup.x + (popup.w - total_width) / 2.0;
    let buttons_y = popup.y + popup.h - button_height - BUTTON_BOTTOM_MARGIN_REF * scale;

    let mut apply_button = Button::new(
        Rect::new(buttons_x, buttons_y, button_width, button_height),
        "Apply",
    );

    let mut close_button = Button::new(
        Rect::new(
            buttons_x + button_width + button_gap,
            buttons_y,
            button_width,
            button_height,
        ),
        "Close",
    );

    let buttons_enabled = !state.resolution_dropdown.open;
    apply_button.enabled = buttons_enabled;
    close_button.enabled = buttons_enabled;

    apply_button.update();
    close_button.update();

    apply_button.draw(font, display);
    close_button.draw(font, display);

    resolution_dropdown.draw(
        &mut state.resolution_dropdown,
        font,
        display,
        |resolution| resolution.label(),
    );

    if apply_button.is_clicked() {
        let selected_resolution = Resolution::ALL[state.selected_resolution_index];
        display.set_resolution(selected_resolution);

        let window_mode = if state.fullscreen {
            WindowMode::Fullscreen
        } else {
            WindowMode::Windowed
        };
        display.set_window_mode(window_mode);
    }

    if close_button.is_clicked() {
        state.close();
    }
}
