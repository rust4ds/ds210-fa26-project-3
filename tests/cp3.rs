//! Checkpoint 3: the 5x5 game, and the round robin.
//!
//!     cargo test --test cp3
//!
//! The 5x5 games start from six fixed wall layouts (`layout::standard_set()`), so a
//! result you see here is one you can reproduce with `cargo run`. Greedy is
//! deterministic, so these games come out the same way every time.

use std::time::Duration;

use tictactoe::agent::{Agent, FirstMove, Greedy};
use tictactoe::board::Board;
use tictactoe::game::{play_game, Outcome};
use tictactoe::layout;
use tictactoe::minimax::{my_heuristic, score_only, Minimax};
use tictactoe::player::Player;
use tictactoe::tournament::{round_robin, Standing};

const LIMIT: Duration = Duration::from_secs(1);

fn finished(x: &dyn Agent, o: &dyn Agent, board: &Board) -> Outcome {
    let record = play_game(x, o, board, LIMIT, false);
    if let Some((player, why)) = record.forfeit {
        panic!(
            "{player} forfeited: {why:?} (slowest moves X {:.2}s, O {:.2}s). Board:\n{}",
            record.slowest_x.as_secs_f64(),
            record.slowest_o.as_secs_f64(),
            record.final_board
        );
    }
    record.outcome
}

mod five_by_five {
    use super::*;

    #[test]
    fn depth_limited_search_finishes_in_time() {
        let agent = Minimax::with_depth(4);
        let record = play_game(&agent, &FirstMove, &layout::open(), LIMIT, false);
        assert!(record.forfeit.is_none(), "depth 4 on an open board should fit in {LIMIT:?}");
    }

    /// Twelve games against Greedy: each standard board once as X and once as O.
    /// Returns (wins, draws, losses) from Minimax's side.
    fn versus_greedy(agent: &Minimax) -> (u32, u32, u32) {
        let mut tally = (0, 0, 0);
        for (i, board) in layout::standard_set().iter().enumerate() {
            let games = [
                (finished(agent, &Greedy, board), Player::X, "X"),
                (finished(&Greedy, agent, board), Player::O, "O"),
            ];
            for (outcome, me, label) in games {
                match outcome {
                    Outcome::Win(p) if p == me => tally.0 += 1,
                    Outcome::Draw => tally.1 += 1,
                    Outcome::Win(_) => {
                        tally.2 += 1;
                        eprintln!("lost to Greedy as {label} on standard board {i}");
                    }
                }
            }
        }
        tally
    }

    #[test]
    fn never_loses_to_greedy_on_standard_boards() {
        let (_, _, losses) = versus_greedy(&Minimax::with_depth(4));
        assert_eq!(losses, 0, "lost {losses} of 12 games to Greedy");
    }

    #[test]
    fn wins_most_games_against_greedy() {
        // A heuristic that only repeats score() wins about nine of these twelve.
        // One that notices an open two-in-a-row wins all of them.
        let (wins, draws, losses) = versus_greedy(&Minimax::with_depth(4));
        assert!(wins >= 10, "{wins} wins, {draws} draws, {losses} losses against Greedy; need 10 wins");
    }

    #[test]
    fn heuristic_agrees_with_score_on_finished_lines() {
        // Whatever else it rewards, a finished run has to count for the right side.
        let x_line = Board::from_rows(&["XXX..", ".....", ".....", "....O", "....O"]);
        let o_line = Board::from_rows(&[".....", "OOO..", "X....", "X....", "....."]);
        assert!(my_heuristic(&x_line) > 0);
        assert!(my_heuristic(&o_line) < 0);
        assert_eq!(my_heuristic(&layout::open()), 0, "an empty board favors nobody");
    }

    #[test]
    fn search_uses_the_heuristic_it_was_given() {
        // A heuristic that loves the top-left corner. With depth 1 nothing else can
        // matter, so an agent built on it has to play there.
        fn corner_lover(board: &Board) -> i32 {
            match board.cell(0, 0) {
                tictactoe::board::Cell::X => 100,
                tictactoe::board::Cell::O => -100,
                _ => 0,
            }
        }
        let agent = Minimax::with_heuristic(1, "corner", corner_lover);
        assert_eq!(agent.choose_move(&layout::open(), LIMIT), (0, 0));
        assert_eq!(agent.name(), "corner(depth 1)");
        // And score_only really is just the score.
        let board = Board::from_rows(&["XXX..", ".....", ".....", ".....", "....O"]);
        assert_eq!(score_only(&board), board.score());
    }

    #[test]
    fn heuristic_sees_more_than_score() {
        // Two in a row with the third cell open, and nothing finished. `score()` says
        // zero here; a heuristic that only repeats `score()` can't tell these apart.
        let x_threat = Board::from_rows(&["XX...", ".....", ".....", ".....", "....O"]);
        let o_threat = Board::from_rows(&["X....", ".....", ".....", ".....", "..OO."]);
        assert!(my_heuristic(&x_threat) > my_heuristic(&o_threat));
    }
}

mod tournament {
    use super::*;

    /// Always plays the last legal move. Loses to FirstMove, so the standings are known.
    struct LastMove;

    impl Agent for LastMove {
        fn name(&self) -> String {
            String::from("LastMove")
        }
        fn choose_move(&self, board: &Board, _: Duration) -> (usize, usize) {
            *board.moves().last().unwrap()
        }
    }

    fn by_name<'a>(standings: &'a [Standing], name: &str) -> &'a Standing {
        standings.iter().find(|s| s.name == name).unwrap_or_else(|| panic!("no standing for {name}"))
    }

    #[test]
    fn every_pair_plays_both_ways_on_every_board() {
        let agents: Vec<Box<dyn Agent>> = vec![Box::new(FirstMove), Box::new(Greedy), Box::new(LastMove)];
        let boards = vec![layout::classic(), layout::classic()];
        let standings = round_robin(&agents, &boards, LIMIT);
        assert_eq!(standings.len(), 3);
        for s in &standings {
            // Two opponents, as X and as O, on two boards.
            assert_eq!(s.games(), 8, "{} played {} games", s.name, s.games());
        }
    }

    #[test]
    fn results_land_on_the_right_agents() {
        // Minimax beats FirstMove as X and as O (checkpoint 2), so the standings are known.
        let agents: Vec<Box<dyn Agent>> = vec![Box::new(FirstMove), Box::new(Minimax::new())];
        let standings = round_robin(&agents, &[layout::classic()], LIMIT);
        let first = by_name(&standings, "FirstMove");
        let minimax = by_name(&standings, &Minimax::new().name());
        assert_eq!((minimax.wins, minimax.draws, minimax.losses), (2, 0, 0));
        assert_eq!((first.wins, first.draws, first.losses), (0, 0, 2));
    }

    #[test]
    fn x_wins_are_credited_to_x() {
        // FirstMove and LastMove each win as X, so a round robin that credits the wrong
        // side still gives each one win... but the loser would have to be O both times.
        let agents: Vec<Box<dyn Agent>> = vec![Box::new(FirstMove), Box::new(LastMove)];
        let standings = round_robin(&agents, &[layout::classic()], LIMIT);
        for s in &standings {
            assert_eq!((s.wins, s.draws, s.losses), (1, 0, 1), "{}", s.name);
        }
    }

    #[test]
    fn standings_are_sorted_by_points() {
        let agents: Vec<Box<dyn Agent>> = vec![
            Box::new(LastMove),
            Box::new(FirstMove),
            Box::new(Greedy),
            Box::new(Minimax::new()),
        ];
        let standings = round_robin(&agents, &[layout::classic()], LIMIT);
        for pair in standings.windows(2) {
            assert!(pair[0].points() >= pair[1].points(), "{} is above {} with fewer points", pair[0].name, pair[1].name);
        }
        assert_eq!(standings[0].name, Minimax::new().name(), "full-depth minimax should top the table");
    }

    #[test]
    fn standard_set_tournament_runs_in_time() {
        let agents: Vec<Box<dyn Agent>> = vec![Box::new(Greedy), Box::new(Minimax::with_depth(4))];
        let standings = round_robin(&agents, &layout::standard_set(), LIMIT);
        let mm = by_name(&standings, &Minimax::with_depth(4).name());
        assert_eq!(mm.games(), 12);
        assert_eq!(mm.losses, 0);
    }
}
