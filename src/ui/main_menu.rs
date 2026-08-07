use macroquad::prelude::*;

use crate::init;
use crate::ui::components::button::Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    None,
    Play,
    Options,
    Quit,
}

pub fn draw_main_menu(font: &Font) -> MainMenuAction {
    let mut action = MainMenuAction::None;

    draw_text_ex(
        "Pentago",
        init::GAME_WIDTH * 0.28,
        init::GAME_HEIGHT * 0.2,
        TextParams {
            font: None,
            font_size: (init::GAME_HEIGHT * 0.2) as u16,
            font_scale: 1.0,
            color: WHITE,
            rotation: 0.0,
            font_scale_aspect: 1.0,
        },
    );

    let button_width = 280.0;
    let button_height = 60.0;
    let button_gap = 20.0;

    let center_x = screen_width() / 2.0;
    let start_y = 260.0;

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

    play_button.draw(font);
    options_button.draw(font);
    quit_button.draw(font);

    if play_button.is_clicked() {
        action = MainMenuAction::Play;
    } else if options_button.is_clicked() {
        action = MainMenuAction::Options;
    } else if quit_button.is_clicked() {
        action = MainMenuAction::Quit;
    }

    action

    // root_ui().push_skin(skin);

    // root_ui().window(
    //     hash!(),
    //     screen.pos_from_middle(0.5, 0.6, 0.4, 0.5),
    //     screen.gen_size(0.4, 0.5),
    //     |ui| {
    //         let window = WindowContext::new(0.4 * init::GAME_WIDTH, 0.5 * init::GAME_HEIGHT);

    //         if widgets::Button::new("Play")
    //             .position(window.pos_from_middle(0.5, 0.25, 0.6, 0.2))
    //             .ui(ui)
    //         {
    //             action = MainMenuAction::Play;
    //         }

    //         if widgets::Button::new("Options")
    //             .position(window.pos_from_middle(0.5, 0.5, 0.7, 0.2))
    //             .ui(ui)
    //         {
    //             action = MainMenuAction::Options;
    //         }

    //         if widgets::Button::new("Quit")
    //             .position(window.pos_from_middle(0.5, 0.75, 0.6, 0.2))
    //             .ui(ui)
    //         {
    //             action = MainMenuAction::Quit;
    //         }
    //     },
    // );

    // root_ui().pop_skin();

    // action
}
