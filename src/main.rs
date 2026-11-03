//! Play a game, or run a tournament, from the command line.
//!
//!     cargo run -- human minimax                  you against your agent, 3x3
//!     cargo run -- minimax greedy --board walls   your agent against Greedy, 5x5 with walls
//!     cargo run -- random first --board open      5x5, no walls
//!     cargo run -- tournament                     everyone against everyone
//!
//! Agents: first, random, greedy, human, minimax. Boards: classic (default), open, walls.
//! Add --depth N to limit how far minimax looks ahead (the default is 4 on 5x5 boards,
//! unlimited on 3x3), --size N to make the open and walls boards bigger than 5, and
//! --limit SECONDS to change the time limit.
//!
//! To pit your own heuristics against each other, see src/bin/arena.rs.

use std::env;
use std::process;
use std::time::Duration;

use rand::RngExt;

use tictactoe::agent::{Agent, FirstMove, Greedy, Human, Random};
use tictactoe::board::Board;
use tictactoe::game::play_game;
use tictactoe::layout;
use tictactoe::minimax::Minimax;
use tictactoe::tournament::{print_standings, round_robin};

const DEFAULT_LIMIT: Duration = Duration::from_secs(1);

fn usage() -> ! {
    eprintln!("usage: cargo run -- <x agent> <o agent> [--board classic|open|walls] [--size N] [--depth N] [--limit SECONDS]");
    eprintln!("       cargo run -- tournament [--board classic|open|walls] [--size N] [--depth N] [--limit SECONDS]");
    eprintln!("agents: first, random, greedy, human, minimax");
    process::exit(2);
}

/// The argument after a flag, or the usage message if there isn't one.
fn value_after(args: &[String], i: usize, flag: &str) -> String {
    match args.get(i) {
        Some(value) => value.clone(),
        None => {
            eprintln!("{flag} needs a value");
            usage()
        }
    }
}

/// Parse a flag's value as a number, or show the usage message.
fn number_after<T: std::str::FromStr>(args: &[String], i: usize, flag: &str) -> T {
    match value_after(args, i, flag).parse() {
        Ok(number) => number,
        Err(_) => {
            eprintln!("{flag} needs a number");
            usage()
        }
    }
}

fn make_agent(name: &str, depth: Option<usize>) -> Box<dyn Agent> {
    match name {
        "first" => Box::new(FirstMove),
        "random" => Box::new(Random),
        "greedy" => Box::new(Greedy),
        "human" => Box::new(Human),
        "minimax" => match depth {
            Some(d) => Box::new(Minimax::with_depth(d)),
            None => Box::new(Minimax::new()),
        },
        _ => {
            eprintln!("unknown agent: {name}");
            usage()
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut positional = Vec::new();
    let mut board_name = String::from("classic");
    let mut depth = None;
    let mut limit = DEFAULT_LIMIT;
    let mut size = 5;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--board" => {
                i += 1;
                board_name = value_after(&args, i, "--board");
            }
            "--size" => {
                i += 1;
                size = number_after(&args, i, "--size");
                if size < 3 {
                    usage();
                }
            }
            "--depth" => {
                i += 1;
                depth = Some(number_after(&args, i, "--depth"));
            }
            "--limit" => {
                i += 1;
                let seconds: f64 = number_after(&args, i, "--limit");
                limit = Duration::from_secs_f64(seconds);
            }
            other => positional.push(other.to_string()),
        }
        i += 1;
    }

    let boards: Vec<Board> = match board_name.as_str() {
        "classic" => vec![layout::classic()],
        "open" => vec![layout::sized(size)],
        "walls" => vec![layout::walls_sized(size, size, rand::rng().random())],
        _ => usage(),
    };
    // Unlimited minimax can't finish a 5x5 board, so default to depth 4 there.
    if depth.is_none() && board_name != "classic" {
        depth = Some(4);
    }

    match positional.as_slice() {
        [word] if word == "tournament" => {
            let agents: Vec<Box<dyn Agent>> = vec![
                make_agent("first", depth),
                make_agent("random", depth),
                make_agent("greedy", depth),
                make_agent("minimax", depth),
            ];
            let boards = if board_name == "walls" && size == 5 { layout::standard_set() } else { boards };
            let standings = round_robin(&agents, &boards, limit);
            print_standings(&standings);
        }
        [x, o] => {
            let x = make_agent(x, depth);
            let o = make_agent(o, depth);
            play_game(&*x, &*o, &boards[0], limit, true);
        }
        _ => usage(),
    }
}
