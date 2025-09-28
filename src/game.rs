#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Empty,
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Cell { state: CellState::Empty }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileRotation {
    Clockwise,
    CounterClockwise,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub cells: [[Cell; 3]; 3],
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            cells: [[Cell::new(); 3]; 3],
        }
    }

    /// Place a marble (if the cell is empty)
    pub fn place(&mut self, row: usize, column: usize, state: CellState) -> Result<(), &'static str> {
        if row >= 3 || column >= 3 {
            return Err("Invalid position");
        }
        if self.cells[row][column].state != CellState::Empty {
            return Err("Cell already occupied");
        }
        self.cells[row][column].state = state;
        Ok(())
    }

    /// Rotate the tile according to provided TileRotation
    pub fn rotate(&mut self, rotation: TileRotation) {
        let mut new_cells = self.cells;
        if rotation == TileRotation::Clockwise {
            for r in 0..3 {
                for c in 0..3 {
                    new_cells[c][2 - r] = self.cells[r][c];
                }
            }
        } else if rotation == TileRotation::CounterClockwise {
            for r in 0..3 {
                for c in 0..3 {
                    new_cells[2 - c][r] = self.cells[r][c];
                }
            }
            
        }
        self.cells = new_cells;
    }
}

// All available player actions on Board

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    tile_row: usize,
    tile_column: usize,
    row: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rotatation {
    tile_row: usize,
    tile_column: usize,
    rotation_orientation: TileRotation,
}


/// The full Pentago board: 2x2 grid of 3x3 tiles (total 6x6)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub tiles: [[Tile; 2]; 2], // 2x2 = 4 tiles
}

impl Board {
    pub fn new() -> Self {
        Board {
            tiles: [[Tile::new(), Tile::new()],
                    [Tile::new(), Tile::new()]],
        }
    }

    
    /// Place a marble directly into a specific tile
    pub fn place(
        &mut self,
        place_action: Placement,
        state: CellState,
    ) -> Result<(), &'static str> {
        if place_action.tile_row >= 2 || place_action.tile_column >= 2 {
            return Err("Invalid tile id");
        }
        self.tiles[place_action.tile_row][place_action.tile_column].place(place_action.row, place_action.column, state)
    }

    /// Rotate a specific tile (quadrant)
    pub fn rotate_tile(
        &mut self,
        rotation_action: Rotatation
    ) -> Result<(), &'static str> {
        if rotation_action.tile_row >= 2 || rotation_action.tile_column >= 2 {
            return Err("Invalid tile position");
        }
        self.tiles[rotation_action.tile_row][rotation_action.tile_column].rotate(rotation_action.rotation_orientation);
        Ok(())
    }

    /// Get the full 6x6 board as a matrix of CellStates
    pub fn to_matrix(&self) -> [[CellState; 6]; 6] {
        let mut matrix = [[CellState::Empty; 6]; 6];
        for tr in 0..2 {
            for tc in 0..2 {
                for r in 0..3 {
                    for c in 0..3 {
                        matrix[tr * 3 + r][tc * 3 + c] =
                            self.tiles[tr][tc].cells[r][c].state;
                    }
                }
            }
        }
        matrix
    }

    /// Check if a player (by CellState) has 5 in a row
    pub fn check_winner(&self, player: CellState) -> bool {
        if player == CellState::Empty {
            return false;
        }

        let board = self.to_matrix();

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

                    for _ in 0..4 { // need 4 more after the first
                        next_row += direction_row;
                        next_column += direction_column;
                        if next_row < 0 || next_row >= 6 || next_column < 0 || next_column >= 6 {
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
    Rotatation(Rotatation),
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

    pub fn place(&mut self) {
        // place 
        // self.last_action = placement
        self.state = TurnState::PlacementDone
    }

    pub fn rotate(&mut self) {
        // rotate 
        // self.last_action = rotation
        self.state = TurnState::RotationDone
    }

    pub fn validate(&mut self) {
        if self.board.check_winner(CellState::White) {
            self.winner = CellState::White;
            return;
        } else if self.board.check_winner(CellState::White) {
            self.winner = CellState::Black;
            return;
        }

        match &self.state {
            TurnState::WaitingForPlacement => {},
            TurnState::WaitingForRotation => {},
            TurnState::PlacementDone => {
                self.state = TurnState::WaitingForRotation
            }
            TurnState::RotationDone => {
                let next_player = match self.current_player {
                    CellState::Black => CellState::White,
                    CellState::White => CellState::Black,
                    CellState::Empty => CellState::Empty,                    
                };
                self.current_player = next_player;
                self.state = TurnState::WaitingForPlacement
            }
        }
    }

    pub fn cancel_action(&mut self) {
        if self.state == TurnState::PlacementDone {
            if let PlayerAction::Placement(ref last_placement) = self.last_action {
                self.board.place(last_placement.clone(), CellState::Empty).unwrap();
            }
        } else if self.state == TurnState::RotationDone {
            if let PlayerAction::Rotatation(ref last_rotation) = self.last_action {
                let opposite_orientation = match last_rotation.rotation_orientation {
                    TileRotation::CounterClockwise => TileRotation::Clockwise,
                    TileRotation::Clockwise => TileRotation::CounterClockwise,
                    
                };
                let opposite_rotation = Rotatation {
                    tile_row: last_rotation.tile_row,
                    tile_column: last_rotation.tile_column,
                    rotation_orientation: opposite_orientation,

                };
                self.board.rotate_tile(opposite_rotation).unwrap();
            }
        }
    }
}