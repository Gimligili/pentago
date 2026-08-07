use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
mod game;
mod graphics;
mod init;
mod position;
mod style;
mod ui;

use init::window_conf;

use crate::position::WindowContext;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    MainMenu,
    Playing,
}


#[macroquad::main(window_conf())]
async fn main() {
    let skin = style::gen_skin().await;
    let game_textures = ui::game_view::GameTextures::load().await;

    let mut app_state = AppState::MainMenu;
    let mut game = game::Game::new();


    game.board.tiles[0][0].cells[0][0].state = game::CellState::White;
    game.board.tiles[0][0].cells[1][1].state = game::CellState::Black;
    game.board.tiles[1][1].cells[2][2].state = game::CellState::White;

    let screen = WindowContext::new(init::GAME_WIDTH, init::GAME_HEIGHT);

    loop {
        clear_background(graphics::BLEU_NUIT);

        match app_state {
            AppState::MainMenu => {
                draw_text_ex("Pentago", init::GAME_WIDTH * 0.28, init::GAME_HEIGHT*0.2, TextParams {
                        font: Some(&load_ttf_font("./ui_assets/button.ttf").await.unwrap()),
                        font_size: (init::GAME_HEIGHT*0.2) as u16,
                        font_scale: get_time().sin() as f32 / 2.0 + 1.0,
                        color: WHITE,
                        rotation: 0.0,
                        font_scale_aspect: 1.0,
                });

                root_ui().push_skin(&skin);

                root_ui().window(hash!(), screen.pos_from_middle(0.5, 0.6, 0.4, 0.5), screen.gen_size(0.4, 0.5), |ui| {
                    let window = WindowContext::new(0.4 * init::GAME_WIDTH, 0.5 * init::GAME_HEIGHT);
                    if widgets::Button::new("Play")
                        .position(window.pos_from_middle(0.5, 0.25, 0.6, 0.2))
                        .ui(ui)
                    {
                        app_state = AppState::Playing;
                    }
                    widgets::Button::new("Options")
                        .position(window.pos_from_middle(0.5, 0.5, 0.7, 0.2))
                        .ui(ui);

                    widgets::Button::new("Quit")
                        .position(window.pos_from_middle(0.5, 0.75, 0.6, 0.2))
                        .ui(ui);
                });
                root_ui().pop_skin();
            }

            AppState::Playing => {
                ui::game_view::draw_game(&game, &game_textures);
            }
        }
        

        next_frame().await;
    }
}
