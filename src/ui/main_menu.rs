use macroquad::prelude::*;

use crate::display::DisplayContext;
use crate::ui::components::button::Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    None,
    Play,
    Options,
    Quit,
}

pub fn draw_main_menu(font: &Font, display: &DisplayContext) -> MainMenuAction {
    let mut action = MainMenuAction::None;

    let title_font_size = (120.0 * display.scale) as u16;
    let title_size = measure_text("Pentago", Some(font), title_font_size, 1.0);

    let title_x = (display.width - title_size.width) / 2.0;
    let title_y = display.y(120.0);

    draw_text_ex(
        "Pentago",
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

    let button_width = 280.0 * display.scale;
    let button_height = 60.0 * display.scale;
    let button_gap = 20.0 * display.scale;

    let center_x = display.width / 2.0;
    let start_y = display.y(260.0);

    let mut play_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y,
            button_width,
            button_height,
        ),
        "Play",
    );

    let mut options_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y + button_height + button_gap,
            button_width,
            button_height,
        ),
        "Options",
    );

    let mut quit_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y + (button_height + button_gap) * 2.0,
            button_width,
            button_height,
        ),
        "Exit",
    );

    play_button.update();
    options_button.update();
    quit_button.update();

    play_button.draw(font, display);
    options_button.draw(font, display);
    quit_button.draw(font, display);

    if play_button.is_clicked() {
        action = MainMenuAction::Play;
    } else if options_button.is_clicked() {
        action = MainMenuAction::Options;
    } else if quit_button.is_clicked() {
        action = MainMenuAction::Quit;
    }

    action
}
