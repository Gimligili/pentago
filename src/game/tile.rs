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
        Cell {
            state: CellState::Empty,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileRotation {
    Clockwise,
    CounterClockwise,
}

impl TileRotation {
    pub fn opposite(self) -> Self {
        match self {
            TileRotation::Clockwise => TileRotation::CounterClockwise,
            TileRotation::CounterClockwise => TileRotation::Clockwise,
        }
    }
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
    pub fn place(
        &mut self,
        row: usize,
        column: usize,
        state: CellState,
    ) -> Result<(), &'static str> {
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
            for (r, _) in original_cells.iter().enumerate() {
                for (c, _) in original_cells[r].iter().enumerate() {
                    self.cells[c][2 - r] = original_cells[r][c];
                }
            }
        } else if rotation == TileRotation::CounterClockwise {
            for (r, _) in original_cells.iter().enumerate() {
                for (c, _) in original_cells[r].iter().enumerate() {
                    self.cells[2 - c][r] = original_cells[r][c];
                }
            }
        }
    }

    pub fn remove(&mut self, row: usize, column: usize) -> Result<(), &'static str> {
        if row >= 3 || column >= 3 {
            return Err("Invalid position");
        }

        if self.cells[row][column].state == CellState::Empty {
            return Err("Cell already empty");
        }

        self.cells[row][column].state = CellState::Empty;

        Ok(())
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_marble() {
        let mut tile = Tile::new();

        assert!(tile.place(1, 1, CellState::White).is_ok());
        assert_eq!(tile.cells[1][1].state, CellState::White);
    }

    #[test]
    fn cannot_place_on_occupied_cell() {
        let mut tile = Tile::new();

        tile.place(1, 1, CellState::White).unwrap();

        assert!(tile.place(1, 1, CellState::Black).is_err());
    }

    #[test]
    fn clockwise_rotation() {
        let mut tile = Tile::new();

        tile.cells[0][0].state = CellState::White;

        tile.rotate(TileRotation::Clockwise);

        assert_eq!(tile.cells[0][2].state, CellState::White);
    }

    #[test]
    fn counter_clockwise_rotation() {
        let mut tile = Tile::new();

        tile.cells[0][0].state = CellState::White;

        tile.rotate(TileRotation::CounterClockwise);

        assert_eq!(tile.cells[2][0].state, CellState::White);
    }
}
