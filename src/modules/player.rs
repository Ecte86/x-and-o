use crate::modules::player_symbol::PlayerSymbol;

use anyhow::{anyhow, bail, Result as AnyResult};
use std::fmt::Display;
use std::iter::Iterator;
use std::slice::IterMut;
use std::vec::Vec;
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct Player {
    symbol: PlayerSymbol,
    number: u8,
}

impl Iterator for Player {
    type Item = PlayerSymbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.symbol == PlayerSymbol::Cross {
            self.symbol = PlayerSymbol::Nought;
        } else {
            self.symbol = PlayerSymbol::Cross;
        }
        Some(self.symbol)
    }
}

pub(crate) trait PlayerIterator {
    fn next_player(&mut self) -> &mut Player;
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self.symbol {
            PlayerSymbol::Cross => "X",
            PlayerSymbol::Nought => "O",
        };
        write!(f, "{}", symbol)
    }
}

impl Player {
    pub(crate) fn new(symbol: PlayerSymbol, number: u8) -> Self {
        Self { symbol, number }
    }
    pub(crate) fn symbol(&self) -> PlayerSymbol {
        self.symbol
    }
    pub(crate) fn set_symbol(&mut self, symbol: PlayerSymbol) {
        self.symbol = symbol;
    }
    pub(crate) fn current_player(&self) -> &Self {
        self
    }
    pub(crate) fn get_player_number(&self) -> u8 {
        self.number
    }
}
