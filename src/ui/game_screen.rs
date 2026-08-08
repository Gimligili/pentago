use macroquad::prelude::*;

use crate::display::DisplayContext;
use crate::game::{self, Game, GameMode, GameStatus, TurnState};

use super::{
    game_view::{self, GameTextures, GameViewState},
    input,
};

pub fn update_game_screen(
    game: &mut Game,
    textures: &GameTextures,
    view_state: &mut GameViewState,
    display: &DisplayContext,
    font: &Font,
) -> bool {
    game_view::draw_game(game, textures, view_state, display, font);

    let ai_turn =
        game.game_mode == GameMode::PlayerVsAI && game.current_player == game::CellState::Black;

    if ai_turn {
        handle_ai_turn(game);
    } else {
        handle_human_turn(game, view_state, display);
    }

    game.game_status != GameStatus::Ongoing
}

fn handle_ai_turn(game: &mut Game) {
    if game.state != TurnState::WaitingForPlacement {
        return;
    }

    let Some(game_move) = game::ai::choose_best_move(&game.board, game.current_player) else {
        return;
    };

    if let Err(error) = game.place(game_move.placement) {
        println!("{error}");
        return;
    }

    if let Err(error) = game.validate() {
        println!("{error}");
        return;
    }

    if game.game_status != GameStatus::Ongoing {
        return;
    }

    if let Err(error) = game.rotate(game_move.rotation) {
        println!("{error}");
        return;
    }

    if let Err(error) = game.validate() {
        println!("{error}");
    }
}

fn handle_human_turn(game: &mut Game, view_state: &mut GameViewState, display: &DisplayContext) {
    if is_mouse_button_pressed(MouseButton::Right) {
        game.cancel_action();
        view_state.selected_tile = None;
        return;
    }

    match game.state {
        TurnState::WaitingForPlacement => {
            if let Some(placement) = input::clicked_placement(display)
                && let Err(error) = game.place(placement)
            {
                println!("{error}");
            }
        }

        TurnState::PlacementDone => {
            if is_key_pressed(KeyCode::Enter)
                && let Err(error) = game.validate()
            {
                println!("{error}");
            }
        }

        TurnState::WaitingForRotation => {
            handle_rotation_selection(game, view_state, display);
        }

        TurnState::RotationDone => {
            if is_key_pressed(KeyCode::Enter)
                && let Err(error) = game.validate()
            {
                println!("{error}");
            }
        }
    }
}

fn handle_rotation_selection(
    game: &mut Game,
    view_state: &mut GameViewState,
    display: &DisplayContext,
) {
    if let Some(clicked_tile) = input::clicked_tile(display) {
        if view_state.selected_tile == Some(clicked_tile) {
            view_state.selected_tile = None;
        } else {
            view_state.selected_tile = Some(clicked_tile);
        }
    }

    if let Some((tile_row, tile_column)) = view_state.selected_tile
        && let Some(rotation_orientation) = input::clicked_rotation(display)
    {
        let rotation = game::Rotation {
            tile_row,
            tile_column,
            rotation_orientation,
        };

        if let Err(error) = game.rotate(rotation) {
            println!("{error}");
        } else {
            view_state.selected_tile = None;
        }
    }
}
