use super::{
    Board, CellState, GameMove,
    rules::{apply_move, check_winner, legal_moves},
};

fn evaluate_window(window: &[CellState], player: CellState) -> i32 {
    let opponent = match player {
        CellState::White => CellState::Black,
        CellState::Black => CellState::White,
        CellState::Empty => return 0,
    };

    let player_count = window.iter().filter(|&&cell| cell == player).count();

    let opponent_count = window.iter().filter(|&&cell| cell == opponent).count();

    // Une ligne contenant les deux couleurs ne peut plus
    // devenir un alignement de 5 pour l'un des joueurs.
    if player_count > 0 && opponent_count > 0 {
        return 0;
    }

    match player_count {
        5 => 100_000,
        4 => 1_000,
        3 => 100,
        2 => 10,
        1 => 1,
        _ => match opponent_count {
            5 => -100_000,
            4 => -1_000,
            3 => -100,
            2 => -10,
            1 => -1,
            _ => 0,
        },
    }
}

pub fn evaluate_board(board: &Board, player: CellState) -> i32 {
    let opponent = match player {
        CellState::White => CellState::Black,
        CellState::Black => CellState::White,
        CellState::Empty => return 0,
    };

    // Les victoires restent prioritaires
    if check_winner(board, player) {
        return 100_000;
    }

    if check_winner(board, opponent) {
        return -100_000;
    }

    let matrix = board.to_matrix();
    let mut score = 0;

    // Horizontal
    for (row, _) in matrix.iter().enumerate() {
        for start_column in 0..=1 {
            let window = [
                matrix[row][start_column],
                matrix[row][start_column + 1],
                matrix[row][start_column + 2],
                matrix[row][start_column + 3],
                matrix[row][start_column + 4],
            ];

            score += evaluate_window(&window, player);
        }
    }

    // Vertical
    for (column, _) in matrix[0].iter().enumerate() {
        for start_row in 0..=1 {
            let window = [
                matrix[start_row][column],
                matrix[start_row + 1][column],
                matrix[start_row + 2][column],
                matrix[start_row + 3][column],
                matrix[start_row + 4][column],
            ];

            score += evaluate_window(&window, player);
        }
    }

    // Diagonal: top-left -> bottom-right
    for start_row in 0..=1 {
        for start_column in 0..=1 {
            let window = [
                matrix[start_row][start_column],
                matrix[start_row + 1][start_column + 1],
                matrix[start_row + 2][start_column + 2],
                matrix[start_row + 3][start_column + 3],
                matrix[start_row + 4][start_column + 4],
            ];

            score += evaluate_window(&window, player);
        }
    }

    // Diagonal: top-right -> bottom-left
    for start_row in 0..=1 {
        for start_column in 4..=5 {
            let window = [
                matrix[start_row][start_column],
                matrix[start_row + 1][start_column - 1],
                matrix[start_row + 2][start_column - 2],
                matrix[start_row + 3][start_column - 3],
                matrix[start_row + 4][start_column - 4],
            ];

            score += evaluate_window(&window, player);
        }
    }

    score
}

pub fn choose_best_move(board: &Board, player: CellState) -> Option<GameMove> {
    let moves = legal_moves(board);

    let mut best_move = None;
    let mut best_score = i32::MIN;

    for game_move in moves {
        let simulated_board = match apply_move(board, &game_move, player) {
            Ok(board) => board,
            Err(_) => continue,
        };

        let score = evaluate_board(&simulated_board, player);

        if score > best_score {
            best_score = score;
            best_move = Some(game_move);
        }
    }

    best_move
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Placement;

    #[test]
    fn ai_finds_immediate_win() {
        let mut board = Board::new();

        // White: W W W | W . .
        //
        // L'IA peut gagner en plaçant une bille en colonne 4.
        for column in 0..4 {
            let placement = Placement {
                tile_row: 0,
                tile_column: column / 3,
                row: 0,
                column: column % 3,
            };

            board.place(&placement, CellState::White).unwrap();
        }

        let best_move = choose_best_move(&board, CellState::White).expect("AI should find a move");

        let result = apply_move(&board, &best_move, CellState::White).unwrap();

        assert!(
            check_winner(&result, CellState::White),
            "AI should choose a winning move"
        );
    }

    #[test]
    fn evaluate_three_aligned_marble() {
        let window = [
            CellState::White,
            CellState::White,
            CellState::White,
            CellState::Empty,
            CellState::Empty,
        ];

        assert_eq!(evaluate_window(&window, CellState::White), 100);
    }

    #[test]
    fn evaluate_blocked_window() {
        let window = [
            CellState::White,
            CellState::White,
            CellState::Black,
            CellState::Empty,
            CellState::Empty,
        ];

        assert_eq!(evaluate_window(&window, CellState::White), 0);
    }

    #[test]
    fn board_with_three_white_marble_has_positive_score() {
        let mut board = Board::new();

        for column in 0..3 {
            board
                .place(
                    &Placement {
                        tile_row: 0,
                        tile_column: column / 3,
                        row: 0,
                        column: column % 3,
                    },
                    CellState::White,
                )
                .unwrap();
        }

        assert!(evaluate_board(&board, CellState::White) > 0);
    }

    #[test]
    fn white_alignment_is_bad_for_black() {
        let mut board = Board::new();

        for column in 0..3 {
            board
                .place(
                    &Placement {
                        tile_row: 0,
                        tile_column: column / 3,
                        row: 0,
                        column: column % 3,
                    },
                    CellState::White,
                )
                .unwrap();
        }

        assert!(evaluate_board(&board, CellState::Black) < 0);
    }

    #[test]
    fn diagonal_alignment_has_positive_score() {
        let mut board = Board::new();

        for i in 0..3 {
            board
                .place(
                    &Placement {
                        tile_row: i / 3,
                        tile_column: i / 3,
                        row: i % 3,
                        column: i % 3,
                    },
                    CellState::White,
                )
                .unwrap();
        }

        assert!(evaluate_board(&board, CellState::White) > 0);
    }

    #[test]
    fn reverse_diagonal_alignment_has_positive_score() {
        let mut board = Board::new();

        for i in 0..3 {
            let row = i;
            let column = 5 - i;

            board
                .place(
                    &Placement {
                        tile_row: row / 3,
                        tile_column: column / 3,
                        row: row % 3,
                        column: column % 3,
                    },
                    CellState::White,
                )
                .unwrap();
        }

        assert!(evaluate_board(&board, CellState::White) > 0);
    }
}
