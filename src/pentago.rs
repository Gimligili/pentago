use macroquad::prelude::*;
mod game;
mod graphics;
mod init;
mod position;
mod style;
mod ui;

use init::window_conf;

use crate::position::WindowContext;
use ui::{game_over::GameOverAction, mode_selection::ModeSelectionAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    MainMenu,
    ModeSelection,
    Playing,
    GameOver,
}

#[macroquad::main(window_conf())]
async fn main() {
    let skin = style::gen_skin().await;
    let game_textures = ui::game_view::GameTextures::load().await;

    let mut app_state = AppState::MainMenu;
    let mut game = game::Game::new(game::GameMode::PlayerVsPlayer);
    let mut game_view_state = ui::game_view::GameViewState::new();

    let screen = WindowContext::new(init::GAME_WIDTH, init::GAME_HEIGHT);

    loop {
        clear_background(graphics::BLEU_NUIT);

        match app_state {
            AppState::MainMenu => {
                if ui::main_menu::draw_main_menu(&skin, &screen) {
                    app_state = AppState::ModeSelection;
                }
            }

            AppState::ModeSelection => {
                match ui::mode_selection::draw_mode_selection(&skin, &screen) {
                    ModeSelectionAction::None => {}
                    ModeSelectionAction::PlayerVsPlayer => {
                        game = game::Game::new(game::GameMode::PlayerVsPlayer);
                        game_view_state = ui::game_view::GameViewState::new();
                        app_state = AppState::Playing;
                    }
                    ModeSelectionAction::PlayerVsAI => {
                        game = game::Game::new(game::GameMode::PlayerVsAI);
                        game_view_state = ui::game_view::GameViewState::new();
                        app_state = AppState::Playing;
                    }
                    ModeSelectionAction::Back => {
                        app_state = AppState::MainMenu;
                    }
                }
            }

            AppState::Playing => {
                if ui::game_screen::update_game_screen(
                    &mut game,
                    &game_textures,
                    &mut game_view_state,
                ) {
                    app_state = AppState::GameOver;
                }
            }

            AppState::GameOver => {
                match ui::game_over::draw_game_over(game.game_status, &skin, &screen) {
                    GameOverAction::None => {}

                    GameOverAction::PlayAgain => {
                        let mode = game.game_mode;

                        game = game::Game::new(mode);
                        game_view_state = ui::game_view::GameViewState::new();

                        app_state = AppState::Playing;
                    }

                    GameOverAction::MainMenu => {
                        game_view_state = ui::game_view::GameViewState::new();
                        app_state = AppState::MainMenu;
                    }
                }
            }
        }

        next_frame().await;
    }
}
