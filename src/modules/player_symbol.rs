use std::fmt::Display;
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum PlayerSymbol {
    Cross,
    Nought,
}

impl PlayerSymbol {
    /// Returns the opposite symbol of the current symbol
    /// E.g.
    /// ```
    ///     Cross.opposite() == Nought
    ///     Nought.opposite() == Cross
    /// ```
    pub(crate) fn opposite(&self) -> Self {
        match self {
            PlayerSymbol::Cross => PlayerSymbol::Nought,
            PlayerSymbol::Nought => PlayerSymbol::Cross,
        }
    }
}

impl Display for PlayerSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let symbol = match self {
            PlayerSymbol::Cross => "X",
            PlayerSymbol::Nought => "O",
        };
        write!(f, "{}", symbol)
    }
}
