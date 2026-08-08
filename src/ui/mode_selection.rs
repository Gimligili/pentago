use macroquad::prelude::*;

use crate::{display::DisplayContext, ui::components::button::Button};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeSelectionAction {
    None,
    PlayerVsPlayer,
    PlayerVsAI,
    Back,
}

pub fn draw_mode_selection(font: &Font, display: &DisplayContext) -> ModeSelectionAction {
    let mut action = ModeSelectionAction::None;

    // Title
    let title_text = "Game mode selection";
    let title_font_size = (50.0 * display.scale) as u16;
    let title_size = measure_text(title_text, Some(font), title_font_size, 1.0);
    let title_x = (display.width - title_size.width) / 2.0;
    let title_y = display.y(120.0);
    draw_text_ex(
        title_text,
        title_x,
        title_y,
        TextParams {
            font: Some(font),
            font_size: title_font_size,
            font_scale: 1.0,
            color: WHITE,
            rotation: 0.0,
            font_scale_aspect: 1.0,
        },
    );

    let button_width = 320.0 * display.scale;
    let button_height = 60.0 * display.scale;
    let button_gap = 20.0 * display.scale;

    let center_x = display.width / 2.0;
    let start_y = display.y(250.0);

    let mut pvp_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y,
            button_width,
            button_height,
        ),
        "Player vs Player",
    );

    let mut ai_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y + button_height + button_gap,
            button_width,
            button_height,
        ),
        "Player vs AI",
    );

    let mut back_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y + (button_height + button_gap) * 2.0,
            button_width,
            button_height,
        ),
        "Back",
    );

    pvp_button.update();
    ai_button.update();
    back_button.update();

    pvp_button.draw(font, display);
    ai_button.draw(font, display);
    back_button.draw(font, display);

    if pvp_button.is_clicked() {
        action = ModeSelectionAction::PlayerVsPlayer;
    } else if ai_button.is_clicked() {
        action = ModeSelectionAction::PlayerVsAI;
    } else if back_button.is_clicked() {
        action = ModeSelectionAction::Back;
    }

    action
}
