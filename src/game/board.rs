use super::{CellState, Tile, TileRotation};

// All available player actions on Board

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    pub tile_row: usize,
    pub tile_column: usize,
    pub row: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rotation {
    pub tile_row: usize,
    pub tile_column: usize,
    pub rotation_orientation: TileRotation,
}

/// The full Pentago board: 2x2 grid of 3x3 tiles (total 6x6)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub tiles: [[Tile; 2]; 2], // 2x2 = 4 tiles
}

impl Board {
    pub fn new() -> Self {
        Board {
            tiles: [[Tile::new(), Tile::new()], [Tile::new(), Tile::new()]],
        }
    }

    /// Place a marble directly into a specific tile
    pub fn place(
        &mut self,
        place_action: &Placement,
        state: CellState,
    ) -> Result<(), &'static str> {
        if place_action.tile_row >= 2 || place_action.tile_column >= 2 {
            return Err("Invalid tile id");
        }
        self.tiles[place_action.tile_row][place_action.tile_column].place(
            place_action.row,
            place_action.column,
            state,
        )
    }

    pub fn remove(&mut self, placement: &Placement) -> Result<(), &'static str> {
        if placement.tile_row >= 2 || placement.tile_column >= 2 {
            return Err("Invalid tile id");
        }
        self.tiles[placement.tile_row][placement.tile_column]
            .remove(placement.row, placement.column)
    }

    /// Rotate a specific tile (quadrant)
    pub fn rotate_tile(&mut self, rotation_action: &Rotation) -> Result<(), &'static str> {
        if rotation_action.tile_row >= 2 || rotation_action.tile_column >= 2 {
            return Err("Invalid tile position");
        }
        self.tiles[rotation_action.tile_row][rotation_action.tile_column]
            .rotate(rotation_action.rotation_orientation);
        Ok(())
    }

    /// Get the full 6x6 board as a matrix of CellStates
    pub fn to_matrix(&self) -> [[CellState; 6]; 6] {
        let mut matrix = [[CellState::Empty; 6]; 6];
        for tr in 0..2 {
            for tc in 0..2 {
                for r in 0..3 {
                    for c in 0..3 {
                        matrix[tr * 3 + r][tc * 3 + c] = self.tiles[tr][tc].cells[r][c].state;
                    }
                }
            }
        }
        matrix
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
