use super::{Board, CellState, GameStatus};

pub fn update_game_status(board: &Board) -> GameStatus {
    let white_wins = check_winner(board, CellState::White);
    let black_wins = check_winner(board, CellState::Black);

    match (white_wins, black_wins) {
        (true, false) => GameStatus::WhiteWins,
        (false, true) => GameStatus::BlackWins,
        (true, true) => GameStatus::Draw,
        (false, false) => {
            let board_matrix = board.to_matrix();
            let board_is_full = board_matrix.iter().flatten().all(|cell| *cell != CellState::Empty);
            
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
