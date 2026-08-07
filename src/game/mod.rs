pub mod board;
pub mod rules;
pub mod tile;

pub use board::{Board, Placement, Rotation};
pub use rules::check_winner;
pub use tile::{CellState, Tile, TileRotation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnState {
    WaitingForPlacement,
    PlacementDone,
    WaitingForRotation,
    RotationDone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerAction {
    Placement(Placement),
    Rotation(Rotation),
    Validate,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub current_player: CellState,
    pub state: TurnState,
    pub last_action: PlayerAction,
    pub winner: CellState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: CellState::White,
            state: TurnState::WaitingForPlacement,
            last_action: PlayerAction::Validate,
            winner: CellState::Empty,
        }
    }

    pub fn place(&mut self, placement: Placement) -> Result<(), &'static str> {
        if self.state == TurnState::WaitingForPlacement {
            self.board.place(&placement, self.current_player)?;
            self.last_action = PlayerAction::Placement(placement);
            self.state = TurnState::PlacementDone;
            Ok(())
        } else {
            Err("Action not allowed in this state of the game !")
        }
    }

    pub fn rotate(&mut self, rotation: Rotation) -> Result<(), &'static str> {
        if self.state == TurnState::WaitingForRotation {
            self.board.rotate_tile(&rotation)?;
            self.last_action = PlayerAction::Rotation(rotation);
            self.state = TurnState::RotationDone;
            Ok(())
        } else {
            Err("Action not allowed in this state of the game !")
        }
    }

    pub fn validate(&mut self) -> Result<(), &'static str> {
        if check_winner(&self.board, CellState::White) {
            self.winner = CellState::White;
            return Ok(());
        } else if check_winner(&self.board, CellState::Black) {
            self.winner = CellState::Black;
            return Ok(());
        }

        match &self.state {
            TurnState::WaitingForPlacement => Err("Action not allowed in this state of the game !"),
            TurnState::WaitingForRotation => Err("Action not allowed in this state of the game !"),
            TurnState::PlacementDone => {
                self.state = TurnState::WaitingForRotation;
                self.last_action = PlayerAction::Validate;
                Ok(())
            }
            TurnState::RotationDone => {
                let next_player = match self.current_player {
                    CellState::Black => CellState::White,
                    CellState::White => CellState::Black,
                    CellState::Empty => CellState::Empty,
                };
                self.current_player = next_player;
                self.state = TurnState::WaitingForPlacement;
                self.last_action = PlayerAction::Validate;
                Ok(())
            }
        }
    }

    pub fn cancel_action(&mut self) {
        match &self.state {
            TurnState::PlacementDone => {
                if let PlayerAction::Placement(ref last_placement) = self.last_action {
                    if let Err(e) = self.board.place(last_placement, CellState::Empty) {
                        eprintln!("Error canceling previous placement: {}", e);
                    }
                    self.state = TurnState::WaitingForPlacement;
                    self.last_action = PlayerAction::Validate;
                }
            }
            TurnState::RotationDone => {
                if let PlayerAction::Rotation(ref last_rotation) = self.last_action {
                    let opposite_orientation = match last_rotation.rotation_orientation {
                        TileRotation::CounterClockwise => TileRotation::Clockwise,
                        TileRotation::Clockwise => TileRotation::CounterClockwise,
                    };
                    let opposite_rotation = Rotation {
                        tile_row: last_rotation.tile_row,
                        tile_column: last_rotation.tile_column,
                        rotation_orientation: opposite_orientation,
                    };
                    if let Err(e) = self.board.rotate_tile(&opposite_rotation) {
                        eprintln!("Error canceling previous rotation: {}", e);
                    }
                    self.state = TurnState::WaitingForRotation;
                    self.last_action = PlayerAction::Validate;
                }
            }
            _ => {} // No action needed for other states
        }
    }
}
