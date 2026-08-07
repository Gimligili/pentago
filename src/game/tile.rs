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
        let original_cells = self.cells;
        if rotation == TileRotation::Clockwise {
            for r in 0..3 {
                for c in 0..3 {
                    self.cells[c][2 - r] = original_cells[r][c];
                }
            }
        } else if rotation == TileRotation::CounterClockwise {
            for r in 0..3 {
                for c in 0..3 {
                    self.cells[2 - c][r] = original_cells[r][c];
                }
            }
            
        }
    }
}