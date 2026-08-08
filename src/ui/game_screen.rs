use macroquad::prelude::*;

use super::{
    game_screen_state::{AiTurnState, GameScreenState, RotationAnimation},
    game_view::{self, GameTextures},
    input,
};
use crate::display::DisplayContext;
use pentago_engine::game::{self, Game, GameMode, GameStatus, TurnState};

const AI_THINKING_DURATION: f32 = 0.5;
const AI_PLACEMENT_DURATION: f32 = 0.6;

fn update_rotation_animation(game: &mut Game, game_screen_state: &mut GameScreenState) {
    let Some(animation) = &mut game_screen_state.rotation_animation else {
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

    game_screen_state.rotation_animation = None;
}

pub fn update_game_screen(
    game: &mut Game,
    textures: &GameTextures,
    game_screen_state: &mut GameScreenState,
    display: &DisplayContext,
    font: &Font,
) -> bool {
    update_rotation_animation(game, game_screen_state);

    game_view::draw_game(game, textures, game_screen_state, display, font);

    // The game is terminal: no more input or AI processing.
    if game.game_status != GameStatus::Ongoing {
        return true;
    }

    if game_screen_state.rotation_animation.is_some() {
        return false;
    }

    let ai_turn =
        game.game_mode == GameMode::PlayerVsAI && game.current_player == game::CellState::Black;

    if ai_turn {
        handle_ai_turn(game, game_screen_state);
    } else {
        handle_human_turn(game, game_screen_state, display);
    }

    game.game_status != GameStatus::Ongoing
}

fn handle_ai_turn(game: &mut Game, screen_state: &mut GameScreenState) {
    match screen_state.ai_turn_state {
        AiTurnState::Idle => {
            if game.state != TurnState::WaitingForPlacement {
                return;
            }

            screen_state.ai_turn_state = AiTurnState::Thinking { timer: 0.0 };
        }

        AiTurnState::Thinking { mut timer } => {
            timer += get_frame_time();

            if timer < AI_THINKING_DURATION {
                screen_state.ai_turn_state = AiTurnState::Thinking { timer };

                return;
            }

            let Some(game_move) = game::ai::choose_best_move(&game.board, game.current_player)
            else {
                return;
            };

            if let Err(error) = game.place(game_move.placement) {
                println!("{error}");
                screen_state.ai_turn_state = AiTurnState::Idle;
                return;
            }

            if let Err(error) = game.validate() {
                println!("{error}");
                screen_state.ai_turn_state = AiTurnState::Idle;
                return;
            }

            if game.game_status != GameStatus::Ongoing {
                screen_state.ai_turn_state = AiTurnState::Idle;
                return;
            }

            screen_state.ai_turn_state = AiTurnState::PlacementShown {
                timer: 0.0,
                rotation: game_move.rotation,
            };
        }

        AiTurnState::PlacementShown {
            mut timer,
            rotation,
        } => {
            timer += get_frame_time();

            if timer < AI_PLACEMENT_DURATION {
                screen_state.ai_turn_state = AiTurnState::PlacementShown { timer, rotation };

                return;
            }

            screen_state.rotation_animation = Some(RotationAnimation {
                tile_row: rotation.tile_row,
                tile_column: rotation.tile_column,
                orientation: rotation.rotation_orientation,
                progress: 0.0,
            });

            screen_state.ai_turn_state = AiTurnState::WaitingForRotationAnimation;
        }

        AiTurnState::WaitingForRotationAnimation => {
            if screen_state.rotation_animation.is_some() {
                return;
            }

            if game.state == TurnState::RotationDone
                && let Err(error) = game.validate()
            {
                println!("{error}");
                return;
            }

            screen_state.ai_turn_state = AiTurnState::Idle;
        }
    }
}

fn handle_human_turn(
    game: &mut Game,
    game_screen_state: &mut GameScreenState,
    display: &DisplayContext,
) {
    if is_mouse_button_pressed(MouseButton::Right) {
        if game.game_status != GameStatus::Ongoing {
            return;
        }
        if game.state == TurnState::RotationDone
            && let game::PlayerAction::Rotation(last_rotation) = &game.last_action
        {
            let opposite_orientation = last_rotation.rotation_orientation.opposite();

            game_screen_state.rotation_animation = Some(RotationAnimation {
                tile_row: last_rotation.tile_row,
                tile_column: last_rotation.tile_column,
                orientation: opposite_orientation,
                progress: 0.0,
            });

            return;
        }

        game.cancel_action();
        game_screen_state.selected_tile = None;
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
            handle_rotation_selection(game_screen_state, display);
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

fn handle_rotation_selection(game_screen_state: &mut GameScreenState, display: &DisplayContext) {
    if let Some(clicked_tile) = input::clicked_tile(display) {
        if game_screen_state.selected_tile == Some(clicked_tile) {
            game_screen_state.selected_tile = None;
        } else {
            game_screen_state.selected_tile = Some(clicked_tile);
        }
    }

    if let Some((tile_row, tile_column)) = game_screen_state.selected_tile
        && let Some(rotation_orientation) = input::clicked_rotation(display)
    {
        game_screen_state.rotation_animation = Some(RotationAnimation {
            tile_row,
            tile_column,
            orientation: rotation_orientation,
            progress: 0.0,
        });

        game_screen_state.selected_tile = None;
    }
}
