use macroquad::prelude::*;
mod display;
mod game;
mod init;
mod ui;

use init::window_conf;

use crate::display::DisplayContext;
use crate::ui::main_menu::MainMenuAction;

use ui::{game_over::GameOverAction, mode_selection::ModeSelectionAction};

pub const BLEU_NUIT: Color = Color::from_rgba(5, 5, 30, 255);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    MainMenu,
    ModeSelection,
    Playing,
    GameOver,
}

#[macroquad::main(window_conf())]
async fn main() {
    let font = load_ttf_font("./ui_assets/button.ttf").await.unwrap();
    let game_textures = ui::game_view::GameTextures::load().await;
    let mut display = DisplayContext::new();
    let mut options_state = ui::options::OptionsPopupState::new(&display);

    request_new_screen_size(1280.0, 720.0);

    let mut app_state = AppState::MainMenu;
    let mut game = game::Game::new(game::GameMode::PlayerVsPlayer);
    let mut game_view_state = ui::game_view::GameViewState::new();

    loop {
        display.refresh();
        clear_background(BLEU_NUIT);

        match app_state {
            AppState::MainMenu => {
                match ui::main_menu::draw_main_menu(&font, &display, !options_state.open) {
                    MainMenuAction::None => {}

                    MainMenuAction::Play => {
                        app_state = AppState::ModeSelection;
                    }

                    MainMenuAction::Options => {
                        options_state.open(&display);
                    }

                    MainMenuAction::Quit => {
                        return;
                    }
                };
                if options_state.open {
                    ui::options::draw_options_popup(&mut options_state, &font, &mut display);
                }
            }

            AppState::ModeSelection => {
                match ui::mode_selection::draw_mode_selection(&font, &display) {
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
                    &display,
                    &font,
                ) {
                    app_state = AppState::GameOver;
                }
            }

            AppState::GameOver => {
                match ui::game_over::draw_game_over(game.game_status, &font, &display) {
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
