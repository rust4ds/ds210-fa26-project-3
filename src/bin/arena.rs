//! The arena: your heuristics against each other. Yours to edit, and not graded.
//!
//!     cargo run --bin arena
//!
//! Every agent listed below plays every other one on the six standard boards, as X and
//! as O, and the standings are printed. Add a version of your heuristic, give it a name,
//! and see whether it climbs the table. Try depths too: a sharper heuristic at depth 2
//! often beats a blind one at depth 4.

use std::time::Duration;

use tictactoe::agent::{Agent, Greedy};
use tictactoe::layout;
use tictactoe::minimax::{my_heuristic, score_only, Minimax};
use tictactoe::tournament::{print_standings, round_robin};

fn main() {
    let agents: Vec<Box<dyn Agent>> = vec![
        Box::new(Greedy),
        Box::new(Minimax::with_heuristic(2, "score only", score_only)),
        Box::new(Minimax::with_heuristic(4, "score only", score_only)),
        Box::new(Minimax::with_heuristic(2, "mine", my_heuristic)),
        Box::new(Minimax::with_heuristic(4, "mine", my_heuristic)),
        // Add more here. Any function with the signature fn(&Board) -> i32 works:
        // Box::new(Minimax::with_heuristic(3, "open twos", open_twos)),
    ];

    // Change the boards too: layout::sized(7), or layout::walls_sized(7, 8, 1).
    let boards = layout::standard_set();

    println!("{} agents, {} boards, every pair both ways...", agents.len(), boards.len());
    let standings = round_robin(&agents, &boards, Duration::from_secs(1));
    print_standings(&standings);
}
