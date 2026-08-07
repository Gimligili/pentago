use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use crate::game::GameStatus;
use crate::init;
use crate::position::WindowContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameOverAction {
    None,
    PlayAgain,
    MainMenu,
}

pub fn draw_game_over(
    game_status: GameStatus,
    skin: &macroquad::ui::Skin,
    screen: &WindowContext,
) -> GameOverAction {
    let message = match game_status {
        GameStatus::WhiteWins => "White wins!",
        GameStatus::BlackWins => "Black wins!",
        GameStatus::Draw => "Draw!",
        GameStatus::Ongoing => "",
    };

    draw_text(message, 260.0, 180.0, 50.0, WHITE);

    let mut action = GameOverAction::None;

    root_ui().push_skin(skin);

    root_ui().window(
        hash!(),
        screen.pos_from_middle(0.5, 0.6, 0.4, 0.35),
        screen.gen_size(0.4, 0.35),
        |ui| {
            let window = WindowContext::new(0.4 * init::GAME_WIDTH, 0.35 * init::GAME_HEIGHT);

            if widgets::Button::new("Play again")
                .position(window.pos_from_middle(0.5, 0.35, 0.7, 0.25))
                .ui(ui)
            {
                action = GameOverAction::PlayAgain;
            }

            if widgets::Button::new("Main menu")
                .position(window.pos_from_middle(0.5, 0.70, 0.7, 0.25))
                .ui(ui)
            {
                action = GameOverAction::MainMenu;
            }
        },
    );

    root_ui().pop_skin();

    action
}
