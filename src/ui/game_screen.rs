use macroquad::prelude::*;

use crate::display::DisplayContext;
use crate::game::{self, Game, GameMode, GameStatus, TurnState};

use super::{
    game_view::{self, GameTextures, GameViewState, RotationAnimation},
    input,
};

fn update_rotation_animation(game: &mut Game, view_state: &mut GameViewState) {
    let Some(animation) = &mut view_state.rotation_animation else {
        return;
    };

    animation.progress += get_frame_time() / game_view::ROTATION_ANIMATION_DURATION;
    animation.progress = animation.progress.min(1.0);
    if animation.progress < 1.0 {
        return;
    }

    match game.state {
        TurnState::WaitingForRotation => {
            let rotation = game::Rotation {
                tile_row: animation.tile_row,
                tile_column: animation.tile_column,
                rotation_orientation: animation.orientation,
            };

            if let Err(error) = game.rotate(rotation) {
                println!("{error}");
            }
        }

        TurnState::RotationDone => {
            game.cancel_action();
        }

        _ => {}
    }

    view_state.rotation_animation = None;
}

pub fn update_game_screen(
    game: &mut Game,
    textures: &GameTextures,
    view_state: &mut GameViewState,
    display: &DisplayContext,
    font: &Font,
) -> bool {
    update_rotation_animation(game, view_state);

    game_view::draw_game(game, textures, view_state, display, font);

    if view_state.rotation_animation.is_some() {
        return game.game_status != GameStatus::Ongoing;
    }

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
        if game.state == TurnState::RotationDone
            && let game::PlayerAction::Rotation(last_rotation) = &game.last_action
        {
            let opposite_orientation = last_rotation.rotation_orientation.opposite();

            view_state.rotation_animation = Some(RotationAnimation {
                tile_row: last_rotation.tile_row,
                tile_column: last_rotation.tile_column,
                orientation: opposite_orientation,
                progress: 0.0,
            });

            return;
        }

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
            handle_rotation_selection(view_state, display);
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

fn handle_rotation_selection(view_state: &mut GameViewState, display: &DisplayContext) {
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
        view_state.rotation_animation = Some(RotationAnimation {
            tile_row,
            tile_column,
            orientation: rotation_orientation,
            progress: 0.0,
        });

        view_state.selected_tile = None;
    }
}
