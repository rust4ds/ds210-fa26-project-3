//! Checkpoint 2: Minimax on the 3x3 board.
//!
//!     cargo test --test cp2
//!
//! Minimax searches the whole game, so against any opponent it never loses, and against
//! a weak one it wins. Random's games are different every time, which is why the tests
//! against it only say "never lose". The final autograder also plays your agent against
//! an opponent that isn't in this repo.

use std::time::Duration;

use tictactoe::agent::{Agent, FirstMove, Greedy, Random};
use tictactoe::board::Board;
use tictactoe::game::{play_game, Outcome};
use tictactoe::layout;
use tictactoe::minimax::Minimax;
use tictactoe::player::Player;

const LIMIT: Duration = Duration::from_secs(2);

fn result(x: &dyn Agent, o: &dyn Agent) -> Outcome {
    let record = play_game(x, o, &layout::classic(), LIMIT, false);
    if let Some((player, why)) = record.forfeit {
        panic!("{player} forfeited: {why:?}. Final board:\n{}", record.final_board);
    }
    record.outcome
}

mod tactics {
    use super::*;

    #[test]
    fn takes_the_win_as_x() {
        let board = Board::from_rows(&["XX.", "OO.", "..."]);
        assert_eq!(Minimax::new().choose_move(&board, LIMIT), (0, 2));
    }

    #[test]
    fn takes_the_win_as_o() {
        let board = Board::from_rows(&["XX.", "OO.", "X.."]);
        assert_eq!(board.to_move(), Player::O);
        assert_eq!(Minimax::new().choose_move(&board, LIMIT), (1, 2));
    }

    #[test]
    fn blocks_as_x() {
        let board = Board::from_rows(&["X..", "OO.", "..X"]);
        assert_eq!(Minimax::new().choose_move(&board, LIMIT), (1, 2));
    }

    #[test]
    fn blocks_as_o() {
        let board = Board::from_rows(&["XX.", ".O.", "..."]);
        assert_eq!(Minimax::new().choose_move(&board, LIMIT), (0, 2));
    }

    #[test]
    fn only_one_move_left() {
        let board = Board::from_rows(&["XOX", "XO.", "OXO"]);
        assert_eq!(Minimax::new().choose_move(&board, LIMIT), (1, 2));
    }

    #[test]
    fn move_is_legal_from_the_opening() {
        let board = layout::classic();
        let position = Minimax::new().choose_move(&board, LIMIT);
        assert!(board.moves().contains(&position));
    }
}

mod games {
    use super::*;

    #[test]
    fn beats_first_move_as_x() {
        assert_eq!(result(&Minimax::new(), &FirstMove), Outcome::Win(Player::X));
    }

    #[test]
    fn beats_first_move_as_o() {
        assert_eq!(result(&FirstMove, &Minimax::new()), Outcome::Win(Player::O));
    }

    #[test]
    fn beats_greedy_as_x() {
        assert_eq!(result(&Minimax::new(), &Greedy), Outcome::Win(Player::X));
    }

    #[test]
    fn beats_greedy_as_o() {
        assert_eq!(result(&Greedy, &Minimax::new()), Outcome::Win(Player::O));
    }

    #[test]
    fn never_loses_to_random_as_x() {
        for _ in 0..20 {
            assert_ne!(result(&Minimax::new(), &Random), Outcome::Win(Player::O));
        }
    }

    #[test]
    fn never_loses_to_random_as_o() {
        for _ in 0..20 {
            assert_ne!(result(&Random, &Minimax::new()), Outcome::Win(Player::X));
        }
    }

    #[test]
    fn draws_against_itself() {
        assert_eq!(result(&Minimax::new(), &Minimax::new()), Outcome::Draw);
    }
}
