//! The board. Checkpoint 1: you write `moves`, `apply_move`, `undo`, `score` and `is_over`.
//!
//! A board is a square grid of cells. X moves first, and the board keeps track of whose
//! turn it is, so a move is just a position: the board knows whose mark to put there.
//! Every move is also pushed onto `history`, so that `undo` can take the last one back.
//!
//! Scoring is the same on every board size: count every run of three X's in a row,
//! column or diagonal, subtract the runs of three O's. On a 3x3 board the game ends the
//! moment that score leaves zero. On a bigger board it runs until the board is full, and
//! the higher count wins.

use std::fmt;

use crate::player::Player;

/// What one cell of the board holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Wall,
    X,
    O,
}

impl Cell {
    /// The cell a player's mark makes.
    pub fn mark(player: Player) -> Cell {
        match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }
}

/// A position on the board as `(row, col)`, both counted from zero at the top left.
pub type Position = (usize, usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    cells: Vec<Vec<Cell>>,
    to_move: Player,
    history: Vec<Position>,
}

impl Board {
    /// A board from a grid of cells. Fails if the grid isn't square.
    pub fn from_cells(cells: Vec<Vec<Cell>>) -> Board {
        let n = cells.len();
        assert!(n >= 3, "a board needs at least three rows");
        for row in &cells {
            assert_eq!(row.len(), n, "a board has to be square");
        }
        // X moves first, so X is to move unless X already has more marks than O.
        let mut xs = 0;
        let mut os = 0;
        for row in &cells {
            for cell in row {
                match cell {
                    Cell::X => xs += 1,
                    Cell::O => os += 1,
                    _ => {}
                }
            }
        }
        let to_move = if xs > os { Player::O } else { Player::X };
        Board { cells, to_move, history: Vec::new() }
    }

    /// A board drawn as text, one string per row. `X` and `O` are marks, `#` is a wall,
    /// and anything else is empty. X is to move unless X has more marks than O.
    ///
    /// ```
    /// use tictactoe::board::Board;
    /// let board = Board::from_rows(&["XX.", ".O.", "..#"]);
    /// ```
    pub fn from_rows(rows: &[&str]) -> Board {
        let mut cells = Vec::new();
        for row in rows {
            let mut cells_in_row = Vec::new();
            for ch in row.chars() {
                let cell = match ch {
                    'X' => Cell::X,
                    'O' => Cell::O,
                    '#' => Cell::Wall,
                    _ => Cell::Empty,
                };
                cells_in_row.push(cell);
            }
            cells.push(cells_in_row);
        }
        Board::from_cells(cells)
    }

    /// The number of rows, which is also the number of columns.
    pub fn size(&self) -> usize {
        self.cells.len()
    }

    /// What is at `(row, col)`.
    pub fn cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    /// Whose turn it is.
    pub fn to_move(&self) -> Player {
        self.to_move
    }

    /// How many moves have been made so far.
    pub fn moves_made(&self) -> usize {
        self.history.len()
    }

    /// Every position a mark can be put in, top to bottom, left to right.
    pub fn moves(&self) -> Vec<Position> {
        // Checkpoint 1. Walk every cell, top to bottom and left to right, and collect
        // the positions of the empty ones. `self.size()` is the number of rows and columns.
        todo!("Checkpoint 1: moves")
    }

    /// Put the current player's mark at `position`, and make it the other player's turn.
    /// Panics if the cell isn't empty.
    pub fn apply_move(&mut self, position: Position) {
        // Checkpoint 1. Three things have to happen, and one check comes first:
        //   1. If the cell isn't empty, panic!, with a message that includes the word
        //      "illegal". Nothing else in the program should ever let that happen, so when
        //      it does you want to hear about it.
        //   2. Put the current player's mark there. `Cell::mark(player)` makes the mark.
        //   3. Remember the move in `self.history`, so `undo` can find it.
        //   4. It is the other player's turn now.
        todo!("Checkpoint 1: apply_move")
    }

    /// Take back the last move. Panics if there is nothing to undo.
    pub fn undo(&mut self) {
        // Checkpoint 1. Everything `apply_move` did, backwards. The last move is at the
        // end of `self.history`. Panic if there isn't one, with the message "nothing to undo".
        todo!("Checkpoint 1: undo")
    }

    /// Runs of three X's minus runs of three O's, over every row, column and diagonal.
    /// Four in a row counts as two runs.
    pub fn score(&self) -> i32 {
        // Checkpoint 1. A run of three is three cells in a straight line that all hold
        // the same mark. Count every run of X, subtract every run of O.
        //
        // Every run has a first cell, and from that cell it goes one of four ways:
        // right, down, down-right, or down-left. (Left and up would count the same runs
        // again from the other end.) So for every cell, look in each of the four
        // directions, and skip any direction where the third cell would be off the board.
        // Row and column indexes are `usize`, so be careful with the down-left direction:
        // subtracting from a `usize` that is already 0 panics.
        todo!("Checkpoint 1: score")
    }

    /// Is the game finished? On a 3x3 board, as soon as someone has three in a row.
    /// On any board, when there are no empty cells left.
    pub fn is_over(&self) -> bool {
        // Checkpoint 1. Two ways for a game to end: there are no moves left, on any board,
        // or someone has three in a row, on a 3x3 board only.
        todo!("Checkpoint 1: is_over")
    }

    /// Who is ahead: `Some(X)` if the score is positive, `Some(O)` if negative, `None` if
    /// tied. Only means "winner" once `is_over()` is true.
    pub fn leader(&self) -> Option<Player> {
        let score = self.score();
        if score > 0 {
            Some(Player::X)
        } else if score < 0 {
            Some(Player::O)
        } else {
            None
        }
    }
}

/// Draws the board, with row and column numbers around the edge, so that
/// `println!("{board}")` works. Written for you; nothing to change here.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.size();
        write!(f, "   ")?;
        for col in 0..n {
            write!(f, " {col}  ")?;
        }
        writeln!(f)?;
        for (row, cells) in self.cells.iter().enumerate() {
            write!(f, "{row}  ")?;
            for (col, cell) in cells.iter().enumerate() {
                let ch = match cell {
                    Cell::Empty => ' ',
                    Cell::Wall => '#',
                    Cell::X => 'X',
                    Cell::O => 'O',
                };
                write!(f, " {ch} ")?;
                if col + 1 < n {
                    write!(f, "|")?;
                }
            }
            writeln!(f)?;
            if row + 1 < n {
                write!(f, "   ")?;
                writeln!(f, "{}", vec!["---"; n].join("+"))?;
            }
        }
        Ok(())
    }
}
