# Project 3: Tic Tac Toe

**Author:** _your name here_

**GitHub username:** _the account your project repos are under_

**Agent name for the tournament:** _letters, digits and underscores, up to 20 characters. Set `AGENT_NAME` in `src/minimax.rs` to match_



### Getting started

You will write a Tic Tac Toe board, an agent that plays it by searching ahead, and a tournament that plays agents against each other. Run everything from this top folder:

```bash
cargo run -- human random                   # play against a random agent, 3x3
cargo run -- human minimax                  # play against your agent, once it exists
cargo run -- minimax greedy --board walls   # your agent against Greedy, 5x5 with walls
cargo run -- tournament --board walls       # everyone against everyone
cargo run --bin arena                       # your heuristics against each other
cargo run -- --help                         # the full list of options
```

Until you write the board, every one of these stops with `not yet implemented`.

### The files

```
src/board.rs        checkpoint 1. The board: moves, apply and undo, score, game over
src/minimax.rs      checkpoint 2, then 3. Your agent
src/tournament.rs   checkpoint 3. The round robin
tests/my_tests.rs   checkpoint 1. Your tests of the board
src/bin/arena.rs    checkpoint 3. A playground for pitting heuristics against each other

src/agent.rs        the Agent trait, and FirstMove, Random, Greedy, and Human
src/game.rs         plays one game between two agents, with a clock
src/layout.rs       starting boards: bigger, with or without walls
src/player.rs       X and O
src/main.rs         the command line
tests/cp1.rs, cp2.rs, cp3.rs   the checkpoint tests
```

**Change only those five files**, plus the writeup at the bottom of this one. Your agent has to compile against our copy of everything else when it enters the tournament, so anything you change outside `minimax.rs` won't come with it. The arena is a scratch pad and isn't graded.


### Running your tests

There is one test file per checkpoint, plus your own. Run them from this top folder:

```bash
cargo test --test cp1        # the board
cargo test --test my_tests   # your tests of the board
cargo test --test cp2        # Minimax on 3x3
cargo test --test cp3        # 5x5, the heuristic, and the round robin
cargo test                   # all of them
```

Add part of a test's name to run just the matching ones: `cargo test --test cp1 undo`.

**If a test hangs**, your search is looking too far ahead. Full minimax on a 5x5 board never finishes, which is what checkpoint 3's depth limit is for. Press Ctrl-C and run the one test on its own.

## Your writeup

Answer the questions from the project handout here, under the headings below. Leave the headings where they are, and leave everything above this line alone. Answers should be about 2-3 sentences per question unless the handout says otherwise.

**Write the answers yourself.** AI can help you understand a question, but the words here have to be yours. The AI citation at the end is required: what you used, for what, and how. If you used none, say so.

### 1. Clone or undo

### 2. One trait, many agents

### 3. Why `Vec<Box<dyn Agent>>`

### 4. How big is the tree

### 5. Your heuristic

### 6. Process

### AI citation
