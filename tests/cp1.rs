//! Checkpoint 1: the board.
//!
//!     cargo test --test cp1
//!
//! You do not write any of these, but read them when one fails. The final autograder
//! runs these again, plus cases that are not here.

use tictactoe::board::{Board, Cell};
use tictactoe::layout;
use tictactoe::player::Player;

mod moves {
    use super::*;

    #[test]
    fn empty_3x3_has_nine_moves() {
        let board = layout::classic();
        let moves = board.moves();
        assert_eq!(moves.len(), 9);
        assert_eq!(moves[0], (0, 0), "moves go top to bottom, left to right");
        assert_eq!(moves[8], (2, 2));
    }

    #[test]
    fn marks_and_walls_are_not_moves() {
        let board = Board::from_rows(&["X.#", ".O.", "#.X"]);
        assert_eq!(board.moves(), vec![(0, 1), (1, 0), (1, 2), (2, 1)]);
    }

    #[test]
    fn full_board_has_no_moves() {
        let board = Board::from_rows(&["XOX", "XOO", "OXX"]);
        assert!(board.moves().is_empty());
    }

    #[test]
    fn five_by_five_with_walls() {
        let board = layout::walls_seeded(5, 1);
        assert_eq!(board.moves().len(), 20);
    }
}

mod apply_and_undo {
    use super::*;

    #[test]
    fn first_move_is_x() {
        let mut board = layout::classic();
        assert_eq!(board.to_move(), Player::X);
        board.apply_move((1, 1));
        assert_eq!(board.cell(1, 1), Cell::X);
        assert_eq!(board.to_move(), Player::O, "after X moves, it is O's turn");
    }

    #[test]
    fn second_move_is_o() {
        let mut board = layout::classic();
        board.apply_move((1, 1));
        board.apply_move((0, 0));
        assert_eq!(board.cell(0, 0), Cell::O);
        assert_eq!(board.to_move(), Player::X);
        assert_eq!(board.moves_made(), 2);
    }

    #[test]
    fn apply_removes_the_move() {
        let mut board = layout::classic();
        board.apply_move((2, 0));
        assert_eq!(board.moves().len(), 8);
        assert!(!board.moves().contains(&(2, 0)));
    }

    #[test]
    #[should_panic(expected = "illegal")]
    fn occupied_cell_is_illegal() {
        let mut board = layout::classic();
        board.apply_move((0, 0));
        board.apply_move((0, 0));
    }

    #[test]
    #[should_panic(expected = "illegal")]
    fn wall_is_illegal() {
        let mut board = Board::from_rows(&["#..", "...", "..."]);
        board.apply_move((0, 0));
    }

    #[test]
    fn undo_restores_the_board() {
        let start = layout::classic();
        let mut board = start.clone();
        board.apply_move((1, 1));
        board.undo();
        assert_eq!(board, start, "after apply then undo, nothing should have changed");
    }

    #[test]
    fn undo_twice_restores_two_moves() {
        let mut board = Board::from_rows(&["X..", "...", "..O"]);
        let start = board.clone();
        board.apply_move((1, 1));
        board.apply_move((0, 2));
        board.undo();
        assert_eq!(board.cell(0, 2), Cell::Empty);
        assert_eq!(board.cell(1, 1), Cell::X);
        assert_eq!(board.to_move(), Player::O);
        board.undo();
        assert_eq!(board, start);
    }

    #[test]
    fn apply_undo_apply_many_times() {
        // What minimax does: try every move from one position, undoing each.
        let start = Board::from_rows(&["X.O", ".X.", "O.."]);
        let mut board = start.clone();
        for position in start.moves() {
            board.apply_move(position);
            assert_eq!(board.cell(position.0, position.1), Cell::X);
            board.undo();
            assert_eq!(board, start);
        }
    }

    #[test]
    #[should_panic(expected = "nothing to undo")]
    fn undo_with_no_history_panics() {
        let mut board = layout::classic();
        board.undo();
    }
}

mod score {
    use super::*;

    #[test]
    fn empty_board_is_zero() {
        assert_eq!(layout::classic().score(), 0);
        assert_eq!(layout::open().score(), 0);
    }

    #[test]
    fn x_row() {
        assert_eq!(Board::from_rows(&["XXX", "OO.", "..."]).score(), 1);
    }

    #[test]
    fn o_column() {
        assert_eq!(Board::from_rows(&["OX.", "OX.", "O.X"]).score(), -1);
    }

    #[test]
    fn both_diagonals() {
        assert_eq!(Board::from_rows(&["X.O", ".X.", "O.X"]).score(), 1);
        assert_eq!(Board::from_rows(&["X.O", ".O.", "OX."]).score(), -1);
    }

    #[test]
    fn two_in_a_row_is_nothing() {
        assert_eq!(Board::from_rows(&["XX.", "OO.", "..."]).score(), 0);
    }

    #[test]
    fn walls_break_lines() {
        // No row or column is complete, but both diagonals run through the center.
        assert_eq!(Board::from_rows(&["X#X", ".X.", "X.X"]).score(), 2);
    }

    #[test]
    fn five_by_five_counts_every_run() {
        // Row 0: X X X X . is two runs of three. Column 0: X X X is one more. O has one column.
        let board = Board::from_rows(&[
            "XXXX.",
            "X.O..",
            "X.O..",
            "..O..",
            ".....",
        ]);
        assert_eq!(board.score(), 3 - 1);
    }

    #[test]
    fn long_diagonal_on_5x5() {
        let board = Board::from_rows(&[
            "....O",
            "...O.",
            "..O..",
            ".O...",
            ".....",
        ]);
        assert_eq!(board.score(), -2, "four on a diagonal is two runs of three");
    }
}

mod game_over {
    use super::*;

    #[test]
    fn fresh_board_is_not_over() {
        assert!(!layout::classic().is_over());
        assert!(!layout::open().is_over());
    }

    #[test]
    fn three_in_a_row_ends_3x3() {
        let board = Board::from_rows(&["XXX", "OO.", "..."]);
        assert!(board.is_over());
        assert_eq!(board.leader(), Some(Player::X));
    }

    #[test]
    fn full_3x3_is_over_even_when_tied() {
        let board = Board::from_rows(&["XOX", "XOO", "OXX"]);
        assert!(board.is_over());
        assert_eq!(board.leader(), None);
    }

    #[test]
    fn three_in_a_row_does_not_end_5x5() {
        let board = Board::from_rows(&[
            "XXX..",
            "OO...",
            ".....",
            ".....",
            ".....",
        ]);
        assert!(!board.is_over(), "on 5x5 the game runs until the board is full");
    }

    #[test]
    fn full_5x5_is_over() {
        let board = Board::from_rows(&[
            "XOXOX",
            "OXOXO",
            "XOXOX",
            "OXOXO",
            "XOXOX",
        ]);
        assert!(board.is_over());
    }
}
