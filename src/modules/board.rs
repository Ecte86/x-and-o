use crate::modules::player::Player;
use anyhow::Result as AnyResult;

use std::fmt::Display;

#[derive(Clone, Copy)]
pub(crate) struct Board {
    board: [[Option<Player>; 3]; 3],
    width: u8,
    height: u8,
    winner: Option<Player>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [[None; 3]; 3],
            width: 3,
            height: 3,
            winner: None,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Add labels to the top of the board
        write!(f, "  ")?;
        for x in 0..self.width {
            write!(f, "{}", x + 1)?;
            if x < self.width - 1 {
                write!(f, "   ")?;
            }
        }
        writeln!(f)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut symbol = match self.board[y as usize][x as usize] {
                    Some(player) => format!("{}", player),
                    None => format!("{}", '.'), // (y * self.width) + x),
                };
                if x == 0 {
                    symbol = format!("{} {}", y + 1, symbol);
                }
                write!(f, "{}", symbol)?;
                if x < self.width - 1 {
                    write!(f, " | ")?;
                }
            }
            writeln!(f)?;
            if y < self.height - 1 {
                writeln!(f, " ---+---+---")?;
            }
        }
        Ok(())
    }
}

impl Board {
    pub(crate) fn new() -> Self {
        let board: Board = Board::default();
        board
    }
    pub(crate) fn set(&mut self, x: u8, y: u8, player: Player) -> AnyResult<()> {
        let (x, y) = if x >= self.width || y >= self.height {
            return Err(anyhow::anyhow!("Invalid position"));
        } else {
            (x as usize, y as usize)
        };
        if self.board[y][x].is_some() {
            return Err(anyhow::anyhow!("Position already taken"));
        }
        self.board[y][x] = Some(player);
        Ok(())
    }
    pub(crate) fn get(&self, x: u8, y: u8) -> Option<Player> {
        let (x, y) = if x >= self.width || y >= self.height {
            return None;
        } else {
            (x as usize, y as usize)
        };
        self.board[y][x]
    }

    /// Check if a vector of 3 cells contain the same player, and are not empty
    /// If both conditions are met, this is a win for that player, so return true
    pub(crate) fn is_win_vec(self, cells: Vec<(u8, u8)>) -> bool {
        let mut player = None;
        // Loop through each cell in the vector
        for (x, y) in cells {
            // Check if the cell is occupied by a player
            if let Some(p) = self.get(x, y) {
                // If the player is None, set it to the current player
                if player.is_none() {
                    player = Some(p);
                } else if player != Some(p) {
                    // If the player is not None,
                    // and the current player is not the same
                    // as the player we found earlier in the vector
                    // then this is not a win so return false
                    return false;
                }
            } else {
                // If the cell is not occupied by a player, return false
                return false;
            }
        }
        true
    }
    /// Check if the game is over
    /// Returns true if the game is over
    /// Uses is_win to check 3 cells at a time to determine if there is a winner
    /// If there is no winner, checks if the board is full
    /// If the board is full, the game is over
    pub(crate) fn is_game_over(&mut self) -> bool {
        // Check if there is a win, if there is, return true
        self.find_win()
    }
    fn set_winner(&mut self, player: Option<Player>) {
        self.winner = player;
    }
    fn find_win(&mut self) -> bool {
        // Check if there is a win
        // Check rows
        for y in 0..self.height {
            let cells = vec![(0, y), (1, y), (2, y)];
            if self.is_win_vec(cells.clone()) {
                if let Some(player) = self.get(cells[0].0, cells[0].1) {
                    self.set_winner(Some(player));
                }
                return true;
            }
        }
        // Check columns
        for x in 0..self.width {
            let cells = vec![(x, 0), (x, 1), (x, 2)];
            if self.is_win_vec(cells.clone()) {
                self.winner = self.get(cells[0].0, cells[0].1);
                return true;
            }
        }
        // Check diagonals
        let cells = vec![(0, 0), (1, 1), (2, 2)];
        if self.is_win_vec(cells.clone()) {
            self.winner = self.get(cells[0].0, cells[0].1);
            return true;
        }
        let cells = vec![(2, 0), (1, 1), (0, 2)];
        if self.is_win_vec(cells.clone()) {
            self.winner = self.get(cells[0].0, cells[0].1);
            return true;
        }
        // Check if the board is full
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y).is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_winner(&mut self) -> Option<Player> {
        // Get the winner, if there is one
        if self.find_win() {
            self.winner
        } else {
            None
        }
    }
}
