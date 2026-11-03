//! The two players. X always moves first.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    /// The other player.
    pub fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}
