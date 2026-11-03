//! Starting boards. `classic()` is the 3x3 game. The others are bigger, with or without
//! walls, and default to 5x5.

use rand::seq::IndexedRandom;
use rand::{RngExt, SeedableRng};

use crate::board::{Board, Cell};

/// An empty 3x3 board.
pub fn classic() -> Board {
    sized(3)
}

/// An empty 5x5 board, no walls.
pub fn open() -> Board {
    sized(5)
}

/// An empty board with `size` rows and columns. 3 is classic, 5 is the project's game,
/// and bigger is possible: try 7 and watch what happens to your search.
pub fn sized(size: usize) -> Board {
    Board::from_cells(vec![vec![Cell::Empty; size]; size])
}

/// A 5x5 board with `count` walls in random cells. Different every call.
pub fn walls(count: usize) -> Board {
    walls_sized(5, count, rand::rng().random())
}

/// A 5x5 board with `count` walls, placed by a seeded random number generator, so the
/// same seed always gives the same board. The tests and the tournament use this.
pub fn walls_seeded(count: usize, seed: u64) -> Board {
    walls_sized(5, count, seed)
}

/// A `size` by `size` board with `count` walls placed by the seed.
pub fn walls_sized(size: usize, count: usize, seed: u64) -> Board {
    assert!(count * 2 < size * size, "that many walls doesn't leave much of a game");
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    // Every position on the board, then pick `count` of them without repeats.
    let mut all = Vec::new();
    for row in 0..size {
        for col in 0..size {
            all.push((row, col));
        }
    }
    let chosen = all.sample(&mut rng, count);
    let mut cells = vec![vec![Cell::Empty; size]; size];
    for (r, c) in chosen {
        cells[*r][*c] = Cell::Wall;
    }
    Board::from_cells(cells)
}

/// Six fixed 5x5 boards with five walls each. Every test and tournament game starts from
/// one of these, so a result can be reproduced.
pub fn standard_set() -> Vec<Board> {
    let mut boards = Vec::new();
    for seed in 1..=6 {
        boards.push(walls_seeded(5, seed));
    }
    boards
}
