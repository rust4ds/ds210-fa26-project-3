//! A round robin: every agent plays every other agent, on every starting board, as both
//! X and O. Checkpoint 3: you write `round_robin`.

use std::time::Duration;

use crate::agent::Agent;
use crate::board::Board;
use crate::game::{play_game, Outcome};
use crate::player::Player;

/// One agent's line in the standings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Standing {
    pub name: String,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
}

impl Standing {
    pub fn new(name: String) -> Standing {
        Standing { name, wins: 0, draws: 0, losses: 0 }
    }

    /// Three for a win, one for a draw.
    pub fn points(&self) -> u32 {
        3 * self.wins + self.draws
    }

    pub fn games(&self) -> u32 {
        self.wins + self.draws + self.losses
    }
}

/// Play every ordered pair of agents on every board in `boards`, the first of the pair as
/// X. Each game is recorded in both agents' standings. Returns the standings sorted by
/// points, highest first, with ties broken by name.
pub fn round_robin(agents: &[Box<dyn Agent>], boards: &[Board], time_limit: Duration) -> Vec<Standing> {
    // Checkpoint 3.
    //   1. One `Standing` per agent, in the same order as `agents`. `Standing::new` takes
    //      the name, and every agent has one: `agents[i].name()`.
    //   2. For every pair of different agents i and j, and every board, play one game
    //      with i as X and j as O. `play_game` wants `&dyn Agent`, and `agents[i]` is a
    //      `Box<dyn Agent>`, so pass `&*agents[i]`: `*` to get at the thing in the box, `&`
    //      to borrow it.
    //   3. Record each result on both agents' lines. `Outcome::Win(Player::X)` means the
    //      agent playing X won, whichever agent that was.
    //   4. Sort by points, highest first. For ties, by name.
    todo!("Checkpoint 3: round_robin")
    }

/// Print standings as a table.
pub fn print_standings(standings: &[Standing]) {
    let width = standings.iter().map(|s| s.name.len()).max().unwrap_or(4).max(4);
    println!("{:>3}  {:<width$}  {:>6}  {:>4}  {:>5}  {:>6}", "#", "Name", "Points", "Wins", "Draws", "Losses");
    for (rank, s) in standings.iter().enumerate() {
        println!(
            "{:>3}  {:<width$}  {:>6}  {:>4}  {:>5}  {:>6}",
            rank + 1, s.name, s.points(), s.wins, s.draws, s.losses
        );
    }
}
