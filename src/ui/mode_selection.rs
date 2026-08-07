use macroquad::prelude::*;

use crate::ui::components::button::Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeSelectionAction {
    None,
    PlayerVsPlayer,
    PlayerVsAI,
    Back,
}

pub fn draw_mode_selection(font: &Font) -> ModeSelectionAction {
    let mut action = ModeSelectionAction::None;

    let button_width = 320.0;
    let button_height = 60.0;
    let button_gap = 20.0;

    let center_x = screen_width() / 2.0;
    let start_y = 220.0;

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

    pvp_button.draw(font);
    ai_button.draw(font);
    back_button.draw(font);

    if pvp_button.is_clicked() {
        action = ModeSelectionAction::PlayerVsPlayer;
    }

    if ai_button.is_clicked() {
        action = ModeSelectionAction::PlayerVsAI;
    }

    if back_button.is_clicked() {
        action = ModeSelectionAction::Back;
    }

    action
}
