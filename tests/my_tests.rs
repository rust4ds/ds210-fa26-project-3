//! Your tests of the board. Checkpoint 1 asks for at least three, on `score`.
//!
//!     cargo test --test my_tests
//!
//! Build a position with `Board::from_rows`, and assert what `score()` should say.
//! Think about what could go wrong in a `score` that passes a few easy cases: a
//! direction it forgets, an edge it runs off, a run it counts twice or not at all.

use tictactoe::board::Board;

#[test]
fn example_a_row_for_x() {
    let board = Board::from_rows(&["XXX", "OO.", "..."]);
    assert_eq!(board.score(), 1);
}

// Your tests go below. Give each one a name that says what it checks.
