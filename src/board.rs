use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub enum TileState {
    Empty,
    Ex,
    Oh,
}

impl From<TileState> for &str {
    fn from(value: TileState) -> Self {
        match value {
            TileState::Empty => "_",
            TileState::Oh => "O",
            TileState::Ex => "X",
        }
    }
}

impl From<&TileState> for &str {
    fn from(value: &TileState) -> Self {
        Self::from(*value)
    }
}

impl Display for TileState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

pub enum BoardUpdateError {
    AlreadyOccupied,
    TileNonExistent,
}

pub struct Board {
    tiles: [[TileState; 3]; 3],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: [[TileState::Empty; 3]; 3],
        }
    }
}

impl Board {
    pub fn update(&mut self, x: usize, y: usize, state: TileState) -> Result<(), BoardUpdateError> {
        let Ok((x, y)) = (match (x, y) {
            (1..=3, 1..=3) => Ok((x - 1, y - 1)),
            _ => Err(BoardUpdateError::TileNonExistent),
        }) else {
            return Err(BoardUpdateError::TileNonExistent);
        };

        let tile = self.tiles[y][x];

        match tile {
            TileState::Empty => Ok(self.tiles[y][x] = state),
            _ => Err(BoardUpdateError::AlreadyOccupied),
        }
    }

    pub fn any_row_won(&self) -> bool {
        if self.any_horizontal_win() || self.any_vertical_win() || self.any_diagonal_win() {
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.tiles = [[TileState::Empty; 3]; 3];
    }

    fn any_horizontal_win(&self) -> bool {
        for x in self.tiles {
            if x[0] == TileState::Empty {
                return false;
            };

            if x[0] == x[1] && x[1] == x[2] {
                return true;
            }
        }

        false
    }

    fn any_vertical_win(&self) -> bool {
        for y in [0, 1, 2] {
            if self.tiles[0][y] == TileState::Empty {
                return false;
            }

            if self.tiles[0][y] == self.tiles[1][y] && self.tiles[1][y] == self.tiles[2][y] {
                return true;
            }
        }

        false
    }

    fn any_diagonal_win(&self) -> bool {
        if self.tiles[0][0] == TileState::Empty || self.tiles[0][2] == TileState::Empty {
            return false;
        }

        if self.tiles[0][0] == self.tiles[1][1] && self.tiles[1][1] == self.tiles[2][2] {
            return true;
        }

        if self.tiles[0][2] == self.tiles[1][1] && self.tiles[1][1] == self.tiles[2][0] {
            return true;
        }

        false
    }
}

impl Display for Board {
    // |_|_|_|
    // |_|_|_|
    // |_|_|_|
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();

        for x in self.tiles {
            for y in x {
                buf = buf + "|" + y.into();
            }
            buf = buf + "|\n";
        }

        f.write_str(&buf)
    }
}
