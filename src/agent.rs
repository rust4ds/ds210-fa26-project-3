//! The `Agent` trait, and four agents to play against.
//!
//! An agent is anything that can look at a board and pick a move. The game loop, the
//! tests and the tournament only ever talk to agents through this trait, which is how a
//! tournament can hold agents of different types in one `Vec<Box<dyn Agent>>`.

use std::io::{self, Write};
use std::time::Duration;

use rand::seq::IndexedRandom;

use crate::board::{Board, Position};
use crate::player::Player;

pub trait Agent {
    /// A short name for standings and transcripts.
    fn name(&self) -> String;

    /// Pick a move for whoever is to move on `board`. The result must be one of
    /// `board.moves()`, and it has to come back within `time_limit`, or the game is
    /// forfeited.
    fn choose_move(&self, board: &Board, time_limit: Duration) -> Position;

    /// Only the `Human` agent gets to take as long as it likes. Leave this alone in your
    /// own agent: the tournament rejects an entry that overrides it.
    fn is_human(&self) -> bool {
        false
    }
}

/// Plays the first legal move it finds: top to bottom, left to right.
pub struct FirstMove;

impl Agent for FirstMove {
    fn name(&self) -> String {
        String::from("FirstMove")
    }

    fn choose_move(&self, board: &Board, _time_limit: Duration) -> Position {
        board.moves()[0]
    }
}

/// Plays a random legal move.
pub struct Random;

impl Agent for Random {
    fn name(&self) -> String {
        String::from("Random")
    }

    fn choose_move(&self, board: &Board, _time_limit: Duration) -> Position {
        *board.moves().choose(&mut rand::rng()).expect("no legal moves")
    }
}

/// Looks one move ahead: tries every legal move and keeps the one that leaves the score
/// best for the player making it. Ties go to the earliest move.
pub struct Greedy;

impl Agent for Greedy {
    fn name(&self) -> String {
        String::from("Greedy")
    }

    fn choose_move(&self, board: &Board, _time_limit: Duration) -> Position {
        let me = board.to_move();
        let mut board = board.clone();
        let mut best_move = None;
        let mut best_score = 0;
        for position in board.moves() {
            board.apply_move(position);
            // From this player's point of view: bigger is better.
            let score = match me {
                Player::X => board.score(),
                Player::O => -board.score(),
            };
            board.undo();
            if best_move.is_none() || score > best_score {
                best_move = Some(position);
                best_score = score;
            }
        }
        best_move.expect("no legal moves")
    }
}

/// Asks whoever is at the keyboard.
pub struct Human;

impl Agent for Human {
    fn name(&self) -> String {
        String::from("You")
    }

    fn is_human(&self) -> bool {
        true
    }

    fn choose_move(&self, board: &Board, _time_limit: Duration) -> Position {
        let moves = board.moves();
        loop {
            print!("Your move, as {} (row col): ", board.to_move());
            io::stdout().flush().expect("could not flush stdout");
            let mut line = String::new();
            io::stdin().read_line(&mut line).expect("could not read a line");
            let numbers: Vec<usize> = line
                .split_whitespace()
                .filter_map(|word| word.parse().ok())
                .collect();
            if numbers.len() == 2 && moves.contains(&(numbers[0], numbers[1])) {
                return (numbers[0], numbers[1]);
            }
            println!("That isn't an empty cell. Type two numbers, like 1 2.");
        }
    }
}
