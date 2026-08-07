pub mod ai;
pub mod board;
pub mod rules;
pub mod tile;

pub use board::{Board, Placement, Rotation};
pub use rules::update_game_status;
pub use tile::{CellState, Tile, TileRotation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    PlayerVsPlayer,
    PlayerVsAI,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    WhiteWins,
    BlackWins,
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameMove {
    pub placement: Placement,
    pub rotation: Rotation,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub game_mode: GameMode,
    pub board: Board,
    pub current_player: CellState,
    pub state: TurnState,
    pub last_action: PlayerAction,
    pub game_status: GameStatus,
}

impl Game {
    pub fn new(game_mode: GameMode) -> Self {
        Game {
            game_mode,
            board: Board::new(),
            current_player: CellState::White,
            state: TurnState::WaitingForPlacement,
            last_action: PlayerAction::Validate,
            game_status: GameStatus::Ongoing,
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
        let game_status = update_game_status(&self.board);
        if game_status != GameStatus::Ongoing {
            self.game_status = game_status;
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
                    match self.board.remove(last_placement) {
                        Ok(()) => {
                            self.state = TurnState::WaitingForPlacement;
                            self.last_action = PlayerAction::Validate;
                        }

                        Err(e) => {
                            eprint!("Error canceling previous placement: {e}")
                        }
                    }
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
