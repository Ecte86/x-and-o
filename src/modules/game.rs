use crate::modules::{
    board::Board, game_mode::GameMode, player::Player, player_symbol::PlayerSymbol,
};
use anyhow::{Result as AnyResult, bail, Error};

pub(crate) struct Game {
    board: Board,
    game_mode: GameMode,
    players: Vec<Player>,
    current_player_index: usize,
    game_over: bool,
}

impl Default for Game {
    fn default() -> Self {
        let plrs = vec![
            Player::new(PlayerSymbol::Cross, 1),
            Player::new(PlayerSymbol::Nought, 2),
        ];
        Self {
            board: Board::new(),
            game_mode: GameMode::TwoPlayer,
            players: plrs,
            game_over: false,
            current_player_index: 0,
        }
    }
}
impl Game {
    /// Creates a new game with default values
    pub(crate) fn new() -> Self {
        self::Game::default()
    }
    /// Gets the current game mode
    pub(crate) fn game_mode(&self) -> &GameMode {
        &self.game_mode
    }
    /// Sets the current game mode
    pub(crate) fn set_game_mode(&mut self, game_mode: GameMode) {
        self.game_mode = game_mode;
    }
    /// Gets the current board as a mutable reference
    pub(crate) fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
    /// Gets the current board as an immutable reference
    pub(crate) fn board(&self) -> &Board {
        &self.board
    }
    /// Gets the current player
    pub(crate) fn current_player(&self) -> Player {
        self.players[self.current_player_index]
    }
    /// Gets the current player as a mutable reference
    pub(crate) fn current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.current_player_index]
    }
    /// Gets the current player's symbol
    pub(crate) fn current_player_symbol(&self) -> PlayerSymbol {
        self.current_player().symbol()
    }
    /// Gets all players as an immutable slice
    /// of length 2.

    pub(crate) fn players(&self) -> &Vec<Player> {
        &self.players
    }
    /// Gets all players as a mutable slice
    /// Returns a mutable reference to the array of players
    pub(crate) fn players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    /// Sets a player's symbol
    /// and sets the next player's symbol
    /// to the opposite of the current player's symbol
    /// E.g. X -> O, O -> X
    pub(crate) fn set_player_symbol(&mut self, current_player: &mut Player, symbol: PlayerSymbol) -> AnyResult<(), Error>{
        if current_player.symbol() == symbol {
            bail!("Player already has that symbol")
        }
        self.current_player_mut().set_symbol(symbol);
        let opp = self.get_next_player_mut();
        match opp {
            Ok(opp) => opp.set_symbol(symbol.opposite()),
            Err(e) => bail!(e),
        };
        Ok(())
    }
    /// Gets the next player
    /// Cycle through the players using an iterator
    /// and return the next one
    pub(crate) fn get_next_player_mut(&mut self) -> anyhow::Result<&mut Player, Error> {
        // iterate through the players
        // if the current player is the last player in the vector,
        // then the next player is the first player in the vector
        if self.players.is_empty() {
            bail!("No players available");
        }
        let mut cur_idx = self.current_player().get_player_number() as usize;
        cur_idx -= 1; // Subtract 1 because the player number is 1-based

        // This will ensure we wrap around if the index goes past the last player.
        cur_idx = (cur_idx + 1) % self.players().len();
        if let Some(player) = self.players_mut().get_mut(cur_idx) {
            Ok(player)
        } else {
            bail!("could not get next player");
        }
    }
    /// Gets the next player
    /// Cycle through the players using an iterator
    /// and return the next one
    pub(crate) fn get_next_player(&self) -> &Player {
        let next_player = self.players.iter().cycle().skip(self.current_player_index + 1).next().unwrap();
        next_player
    }
    pub(crate) fn game_over(&self) -> bool {
        self.game_over
    }
    pub(crate) fn set_game_over(&mut self, game_over: bool) {
        self.game_over = game_over;
    }
    pub(crate) fn winner(&mut self) -> Option<Player> {
        self.board.get_winner()
    }
}
