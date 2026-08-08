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

    let button_width = 320.0 * display.scale;
    let button_height = 60.0 * display.scale;
    let button_gap = 20.0 * display.scale;

    let center_x = display.width / 2.0;
    let start_y = display.y(220.0);

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
