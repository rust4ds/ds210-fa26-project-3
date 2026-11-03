//! Playing one game between two agents, with a clock.

use std::time::{Duration, Instant};

use crate::agent::Agent;
use crate::board::Board;
use crate::player::Player;

/// How one game ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Win(Player),
    Draw,
}

/// Why a game ended early.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Forfeit {
    /// The agent took longer than the time limit.
    TooSlow,
    /// The agent returned a position that wasn't an empty cell.
    IllegalMove,
}

/// Everything worth knowing about a finished game.
#[derive(Clone, Debug)]
pub struct GameRecord {
    pub outcome: Outcome,
    /// The board as the game ended.
    pub final_board: Board,
    /// Set if the loser forfeited, and why.
    pub forfeit: Option<(Player, Forfeit)>,
    /// The slowest move each side made.
    pub slowest_x: Duration,
    pub slowest_o: Duration,
}

/// Play one game from `start`, X moving first. Each move is timed, and an agent that
/// runs over `time_limit` or plays an illegal move loses on the spot. With `verbose`
/// the board is printed after every move.
pub fn play_game(
    x: &dyn Agent,
    o: &dyn Agent,
    start: &Board,
    time_limit: Duration,
    verbose: bool,
) -> GameRecord {
    let mut board = start.clone();
    let mut slowest_x = Duration::ZERO;
    let mut slowest_o = Duration::ZERO;

    if verbose {
        println!("{} is X, {} is O", x.name(), o.name());
        println!("{board}");
    }

    while !board.is_over() {
        let player = board.to_move();
        let agent = match player {
            Player::X => x,
            Player::O => o,
        };

        let started = Instant::now();
        let position = agent.choose_move(&board, time_limit);
        let took = started.elapsed();

        let slowest = match player {
            Player::X => &mut slowest_x,
            Player::O => &mut slowest_o,
        };
        if took > *slowest {
            *slowest = took;
        }

        let forfeit = if took > time_limit && !agent.is_human() {
            Some(Forfeit::TooSlow)
        } else if !board.moves().contains(&position) {
            Some(Forfeit::IllegalMove)
        } else {
            None
        };
        if let Some(reason) = forfeit {
            if verbose {
                match reason {
                    Forfeit::TooSlow => println!(
                        "{} took {:.2}s, over the {:.2}s limit. {} forfeits.",
                        agent.name(),
                        took.as_secs_f64(),
                        time_limit.as_secs_f64(),
                        player
                    ),
                    Forfeit::IllegalMove => println!(
                        "{} played ({}, {}), which isn't empty. {} forfeits.",
                        agent.name(),
                        position.0,
                        position.1,
                        player
                    ),
                }
            }
            return GameRecord {
                outcome: Outcome::Win(player.other()),
                final_board: board,
                forfeit: Some((player, reason)),
                slowest_x,
                slowest_o,
            };
        }

        board.apply_move(position);
        if verbose {
            println!(
                "{} plays ({}, {}) in {:.3}s",
                player, position.0, position.1,
                took.as_secs_f64()
            );
            println!("{board}");
        }
    }

    let outcome = match board.leader() {
        Some(player) => Outcome::Win(player),
        None => Outcome::Draw,
    };
    if verbose {
        match outcome {
            Outcome::Win(player) => println!("{player} wins, score {}", board.score()),
            Outcome::Draw => println!("Draw"),
        }
        println!(
            "Slowest moves: X {:.3}s, O {:.3}s",
            slowest_x.as_secs_f64(),
            slowest_o.as_secs_f64()
        );
    }
    GameRecord { outcome, final_board: board, forfeit: None, slowest_x, slowest_o }
}
