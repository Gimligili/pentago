use super::{Board, CellState, GameMove, GameStatus, Placement, Rotation, TileRotation};

pub fn update_game_status(board: &Board) -> GameStatus {
    let white_wins = check_winner(board, CellState::White);
    let black_wins = check_winner(board, CellState::Black);

    match (white_wins, black_wins) {
        (true, false) => GameStatus::WhiteWins,
        (false, true) => GameStatus::BlackWins,
        (true, true) => GameStatus::Draw,
        (false, false) => {
            let board_matrix = board.to_matrix();
            let board_is_full = board_matrix
                .iter()
                .flatten()
                .all(|cell| *cell != CellState::Empty);

            if board_is_full {
                GameStatus::Draw
            } else {
                GameStatus::Ongoing
            }
        }
    }
}

/// Check if a player (by CellState) has 5 in a row
pub fn check_winner(board: &Board, player: CellState) -> bool {
    if player == CellState::Empty {
        return false;
    }

    let board = board.to_matrix();

    // directions: right, down, down-right diag, down-left diag
    let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];

    for row in 0..6 {
        for column in 0..6 {
            if board[row][column] != player {
                continue;
            }
            for &(direction_row, direction_column) in &directions {
                let mut count = 1;
                let mut next_row = row as isize;
                let mut next_column = column as isize;

                for _ in 0..4 {
                    // need 4 more after the first
                    next_row += direction_row;
                    next_column += direction_column;
                    if !(0..6).contains(&next_row) || !(0..6).contains(&next_column) {
                        break;
                    }
                    if board[next_row as usize][next_column as usize] == player {
                        count += 1;
                    } else {
                        break;
                    }
                }

                if count >= 5 {
                    return true;
                }
            }
        }
    }
    false
}

pub fn legal_moves(board: &Board) -> Vec<GameMove> {
    let mut moves = Vec::new();

    for tile_row in 0..2 {
        for tile_column in 0..2 {
            for row in 0..3 {
                for column in 0..3 {
                    if board.tiles[tile_row][tile_column].cells[row][column].state
                        != CellState::Empty
                    {
                        continue;
                    }

                    let placement = Placement {
                        tile_row,
                        tile_column,
                        row,
                        column,
                    };

                    for rotation_tile_row in 0..2 {
                        for rotation_tile_column in 0..2 {
                            for rotation_orientation in
                                [TileRotation::Clockwise, TileRotation::CounterClockwise]
                            {
                                moves.push(GameMove {
                                    placement: placement.clone(),
                                    rotation: Rotation {
                                        tile_row: rotation_tile_row,
                                        tile_column: rotation_tile_column,
                                        rotation_orientation,
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    moves
}

pub fn apply_move(
    board: &Board,
    game_move: &GameMove,
    player: CellState,
) -> Result<Board, &'static str> {
    let mut new_board = board.clone();

    new_board.place(&game_move.placement, player)?;

    new_board.rotate_tile(&game_move.rotation)?;

    Ok(new_board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Placement;

    fn place(board: &mut Board, row: usize, column: usize, state: CellState) {
        let placement = Placement {
            tile_row: row / 3,
            tile_column: column / 3,
            row: row % 3,
            column: column % 3,
        };

        board.place(&placement, state).unwrap();
    }

    #[test]
    fn horizontal_win() {
        let mut board = Board::new();

        for column in 0..5 {
            place(&mut board, 2, column, CellState::White);
        }

        assert!(check_winner(&board, CellState::White));
    }

    #[test]
    fn vertical_win() {
        let mut board = Board::new();

        for row in 0..5 {
            place(&mut board, row, 4, CellState::Black);
        }

        assert!(check_winner(&board, CellState::Black));
    }

    #[test]
    fn diagonal_win() {
        let mut board = Board::new();

        for i in 0..5 {
            place(&mut board, i, i, CellState::White);
        }

        assert!(check_winner(&board, CellState::White));
    }

    #[test]
    fn four_is_not_enough() {
        let mut board = Board::new();

        for column in 0..4 {
            place(&mut board, 1, column, CellState::White);
        }

        assert!(!check_winner(&board, CellState::White));
    }

    #[test]
    fn simultaneous_win_is_draw() {
        let mut board = Board::new();

        for column in 0..5 {
            place(&mut board, 0, column, CellState::White);
            place(&mut board, 5, column, CellState::Black);
        }

        assert_eq!(update_game_status(&board), GameStatus::Draw);
    }

    #[test]
    fn apply_move_does_not_modify_original_board() {
        let board = Board::new();

        let game_move = GameMove {
            placement: Placement {
                tile_row: 0,
                tile_column: 0,
                row: 0,
                column: 0,
            },
            rotation: Rotation {
                tile_row: 0,
                tile_column: 0,
                rotation_orientation: TileRotation::Clockwise,
            },
        };

        let new_board = apply_move(&board, &game_move, CellState::White).unwrap();

        assert_eq!(board.tiles[0][0].cells[0][0].state, CellState::Empty);

        assert_ne!(new_board, board);
    }
}
