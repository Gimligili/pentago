use pyo3::prelude::*;

use crate::game::{
    CellState, Game, GameMode, GameStatus,
    ai::{action_id_to_move, choose_best_move, move_to_action_id},
    rules::legal_moves,
};

#[pyclass]
pub struct PyGame {
    game: Game,
}

#[pymethods]
impl PyGame {
    #[new]
    fn new() -> Self {
        Self {
            game: Game::new(GameMode::PlayerVsAI),
        }
    }

    fn reset(&mut self) {
        self.game = Game::new(GameMode::PlayerVsAI);
    }

    fn board(&self) -> Vec<Vec<i8>> {
        let matrix = self.game.board.to_matrix();

        matrix
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        CellState::Empty => 0,
                        CellState::White => 1,
                        CellState::Black => -1,
                    })
                    .collect()
            })
            .collect()
    }

    fn current_player(&self) -> i8 {
        match self.game.current_player {
            CellState::White => 1,
            CellState::Black => -1,
            CellState::Empty => 0,
        }
    }

    fn game_status(&self) -> i8 {
        match self.game.game_status {
            GameStatus::Ongoing => 0,
            GameStatus::WhiteWins => 1,
            GameStatus::BlackWins => -1,
            GameStatus::Draw => 2,
        }
    }

    fn legal_actions(&self) -> Vec<usize> {
        if self.game.game_status != GameStatus::Ongoing {
            return Vec::new();
        }

        legal_moves(&self.game.board)
            .iter()
            .map(move_to_action_id)
            .collect()
    }
    fn step(&mut self, action_id: usize) -> PyResult<(f32, bool)> {
        let game_move = action_id_to_move(action_id).ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err(format!("Invalid action id: {action_id}"))
        })?;

        let acting_player = self.game.current_player;

        self.game
            .play_move(game_move)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        let done = self.game.game_status != GameStatus::Ongoing;

        let reward = match self.game.game_status {
            GameStatus::WhiteWins if acting_player == CellState::White => 1.0,
            GameStatus::BlackWins if acting_player == CellState::Black => 1.0,

            GameStatus::WhiteWins | GameStatus::BlackWins => -1.0,

            GameStatus::Draw | GameStatus::Ongoing => 0.0,
        };

        Ok((reward, done))
    }

    fn teacher_action(&self) -> Option<usize> {
        if self.game.game_status != GameStatus::Ongoing {
            return None;
        }

        choose_best_move(&self.game.board, self.game.current_player)
            .map(|game_move| move_to_action_id(&game_move))
    }
}

#[pymodule]
fn pentago_engine(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyGame>()?;

    Ok(())
}
