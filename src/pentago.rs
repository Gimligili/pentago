use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
mod game;
mod graphics;
mod init;
mod position;
mod style;
mod ui;

use init::window_conf;

use crate::game::{Placement, TurnState};
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
    let mut game_view_state = ui::game_view::GameViewState::new();

    game.board.tiles[0][0].cells[0][0].state = game::CellState::White;
    game.board.tiles[0][0].cells[1][1].state = game::CellState::Black;
    game.board.tiles[1][1].cells[2][2].state = game::CellState::White;

    let screen = WindowContext::new(init::GAME_WIDTH, init::GAME_HEIGHT);

    loop {
        clear_background(graphics::BLEU_NUIT);

        match app_state {
            AppState::MainMenu => {
                draw_text_ex(
                    "Pentago",
                    init::GAME_WIDTH * 0.28,
                    init::GAME_HEIGHT * 0.2,
                    TextParams {
                        font: Some(&load_ttf_font("./ui_assets/button.ttf").await.unwrap()),
                        font_size: (init::GAME_HEIGHT * 0.2) as u16,
                        font_scale: get_time().sin() as f32 / 2.0 + 1.0,
                        color: WHITE,
                        rotation: 0.0,
                        font_scale_aspect: 1.0,
                    },
                );

                root_ui().push_skin(&skin);

                root_ui().window(
                    hash!(),
                    screen.pos_from_middle(0.5, 0.6, 0.4, 0.5),
                    screen.gen_size(0.4, 0.5),
                    |ui| {
                        let window =
                            WindowContext::new(0.4 * init::GAME_WIDTH, 0.5 * init::GAME_HEIGHT);
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
                    },
                );
                root_ui().pop_skin();
            }

            AppState::Playing => {
                ui::game_view::draw_game(&game, &game_textures, &game_view_state);

                // Right click: cancel the last action
                if is_mouse_button_pressed(MouseButton::Right) {
                    game.cancel_action();
                    game_view_state.selected_tile = None;
                } else {
                    match game.state {
                        TurnState::WaitingForPlacement => {
                            if let Some(placement) = ui::input::clicked_placement()
                                && let Err(error) = game.place(placement)
                            {
                                println!("{error}");
                            }
                        }

                        TurnState::PlacementDone => {
                            if is_key_pressed(KeyCode::Enter)
                                && let Err(error) = game.validate() {
                                println!("{error}");
                            }
                        }

                        TurnState::WaitingForRotation => {
                            if let Some(clicked_tile) = ui::input::clicked_tile() {
                                if game_view_state.selected_tile == Some(clicked_tile) {
                                    game_view_state.selected_tile = None;
                                } else {
                                    game_view_state.selected_tile = Some(clicked_tile);
                                }
                            }
                            if let Some((tile_row, tile_column)) = game_view_state.selected_tile
                                && let Some(rotation_orientation) = ui::input::clicked_rotation()
                            {
                                let rotation = game::Rotation {
                                    tile_row,
                                    tile_column,
                                    rotation_orientation,
                                };

                                if let Err(error) = game.rotate(rotation) {
                                    println!("{error}");
                                } else {
                                    game_view_state.selected_tile = None
                                }
                            }
                        }

                        TurnState::RotationDone => {
                            if is_key_pressed(KeyCode::Enter)
                                && let Err(error) = game.validate() {
                                println!("{error}");
                            }
                        }
                    }
                }
            }
        }

        next_frame().await;
    }
}
