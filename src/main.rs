// Noughts and Crosses game with a simple AI opponent

use anyhow::Result as AnyResult;
use std::{fmt::Display, io};

#[derive(Copy, Clone, PartialEq)]
enum Player {
    Cross,
    Nought,
}

impl Player {
    fn opponent(&self) -> Player {
        match self {
            Player::Cross => Player::Nought,
            Player::Nought => Player::Cross,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self {
            Player::Cross => "X",
            Player::Nought => "O",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Copy)]
struct Board {
    board: [[Option<Player>; 3]; 3],
    width: u8,
    height: u8,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [[None; 3]; 3],
            width: 3,
            height: 3,
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
        write!(f, "\n")?;
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
            write!(f, "\n")?;
            if y < self.height - 1 {
                write!(f, " ---+---+---\n")?;
            }
        }
        Ok(())
    }
}

impl Board {
    fn new() -> Self {
        let board: Board = Board::default();
        board
    }
    fn set(&mut self, x: u8, y: u8, player: Player) -> AnyResult<()> {
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
    fn get(&self, x: u8, y: u8) -> Option<Player> {
        let (x, y) = if x >= self.width || y >= self.height {
            return None;
        } else {
            (x as usize, y as usize)
        };
        self.board[y][x]
    }

    /// Check if a vector of 3 cells contain the same player, and are not empty
    /// If both conditions are met, this is a win for that player, so return true
    fn is_win(self, cells: Vec<(u8, u8)>) -> bool {
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
    fn is_game_over(&self) -> bool {
        // Check if there is a win
        // Check rows
        for y in 0..self.height {
            let cells = vec![(0, y), (1, y), (2, y)];
            if self.is_win(cells) {
                return true;
            }
        }
        // Check columns
        for x in 0..self.width {
            let cells = vec![(x, 0), (x, 1), (x, 2)];
            if self.is_win(cells) {
                return true;
            }
        }
        // Check diagonals
        let cells = vec![(0, 0), (1, 1), (2, 2)];
        if self.is_win(cells) {
            return true;
        }
        let cells = vec![(2, 0), (1, 1), (0, 2)];
        if self.is_win(cells) {
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
}

fn main() -> AnyResult<()> {
    run()?;
    Ok(())
}
fn run() -> AnyResult<()> {
    println!("Noughts and Crosses\n");
    println!("Enter coordinates in the format x,y.");
    let mut board = Board::new();
    let mut player = Player::Cross;
    let mut game_over = false;
     let board_width_range = 0..3; // 0, 1, 2
     let board_height_range = 0..3; // 0, 1, 2
    //let input_width_range = 1..=3; // 1, 2, 3
    //let input_height_range = 1..=3; // 1, 2, 3

    while !game_over {
        println!("{}", board);
        println!("Player {}'s turn: ", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.starts_with("q") {
            return Ok(());
        }
        let mut x: u8 = 255;
        let mut y: u8 = 255;
        let mut valid_input = false;
        let mut ask_again = false;
        while !valid_input {
            if ask_again {
                println!("Player {}'s turn: ", player);
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
            }
            let input = parse_input(input.trim());
            if let Err(e) = input {
                println!("{}", e);
                continue;
            } else {
                (x, y) = input.unwrap();
            };
            if !(board_width_range).contains(&x) || !(board_height_range).contains(&y) {
                println!("Invalid input");
                valid_input = false;
                ask_again=true;
                continue;
            } else {
                valid_input = true;
            }
        }
        println!("x: {}, y: {}", x, y);
        board.set(x, y, player)?;
        game_over = board.is_game_over();
        if game_over {
            break;
        }
        player = player.opponent();
    }

    println!("{}", board);
    println!("Player {} wins!", player);
    Ok(())
}

fn parse_input(input: &str) -> AnyResult<(u8, u8)> {
    let input = input.trim().replace(" ", "");
    let mut parts = input.split(",").collect::<Vec<&str>>();
    let x = parts
        .pop()
        .ok_or(anyhow::anyhow!("Missing x coordinate"))?
        .to_string();
    let y = parts
        .pop()
        .ok_or(anyhow::anyhow!("Missing y coordinate"))?
        .to_string();
    let x = x.parse::<u8>()?;
    let y = y.parse::<u8>()?;
    if (1..=3).contains(&x) && (1..=3).contains(&y) {
        Ok((x - 1, y - 1))
    } else {
        Err(anyhow::anyhow!("Invalid input"))
    }
}
