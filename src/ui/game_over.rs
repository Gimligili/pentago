use macroquad::prelude::*;

use crate::game::GameStatus;
use crate::ui::components::button::Button;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOverAction {
    None,
    PlayAgain,
    MainMenu,
}

pub fn draw_game_over(game_status: GameStatus, font: &Font) -> GameOverAction {
    let mut action = GameOverAction::None;

    let message = match game_status {
        GameStatus::WhiteWins => "White wins!",
        GameStatus::BlackWins => "Black wins!",
        GameStatus::Draw => "Draw!",
        GameStatus::Ongoing => "",
    };

    let title_font_size = 50;

    let title_size = measure_text(message, Some(font), title_font_size, 1.0);

    draw_text_ex(
        message,
        (screen_width() - title_size.width) / 2.0,
        180.0,
        TextParams {
            font: Some(font),
            font_size: title_font_size,
            color: WHITE,
            ..Default::default()
        },
    );

    let button_width = 280.0;
    let button_height = 60.0;
    let button_gap = 20.0;

    let center_x = screen_width() / 2.0;
    let start_y = 260.0;

    let mut play_again_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y,
            button_width,
            button_height,
        ),
        "Play again",
    );

    let mut main_menu_button = Button::new(
        Rect::new(
            center_x - button_width / 2.0,
            start_y + button_height + button_gap,
            button_width,
            button_height,
        ),
        "Main menu",
    );

    play_again_button.update();
    main_menu_button.update();

    play_again_button.draw(font);
    main_menu_button.draw(font);

    if play_again_button.is_clicked() {
        action = GameOverAction::PlayAgain;
    }

    if main_menu_button.is_clicked() {
        action = GameOverAction::MainMenu;
    }

    action
}
