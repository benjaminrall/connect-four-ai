# Connect Four AI

[![Crates.io Version](https://img.shields.io/crates/v/connect-four-ai)](https://crates.io/crates/connect-four-ai)
[![PyPI Version](https://img.shields.io/pypi/v/connect-four-ai)](https://pypi.org/project/connect-four-ai)
[![NPM Version](https://img.shields.io/npm/v/connect-four-ai-wasm)](https://www.npmjs.com/package/connect-four-ai-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/benjaminrall/connect-four-ai/blob/main/LICENSE)
[![docs.rs](https://img.shields.io/docsrs/connect-four-ai)](https://docs.rs/connect-four-ai)

A high-performance, perfect Connect Four solver written in Rust, with bindings for Python and WebAssembly.

![Connect Four GIF](https://github.com/user-attachments/assets/bb7dff1f-3a27-4f0a-b6ab-b46f19df6fd6)

This library can strongly solve any Connect Four position and determine the optimal move.
For full details, performance benchmarks, and demos, please see the main 
[GitHub Repository](https://github.com/benjaminrall/connect-four-ai).

## Key Features

- **Perfect Solver**: Implements an optimised negamax search,
  which utilises alpha-beta pruning and a transposition table
  to quickly converge on exact game outcomes.

- **AI Player**: Features an AI player with configurable difficulty. It can play
  perfectly by always choosing the optimal move, or can simulate a range of
  skill levels by probabilistically selecting moves based on their scores.

- **Bitboard Representation**: Uses a compact and efficient bitboard representation for
  game positions, allowing for fast move generation and evaluation.

- **Embedded Opening Book**: Includes a pre-generated opening book of depth 8, which is
  embedded directly into the binary for instant lookups of early-game solutions.

- **Parallel Book Generator**: A tool built with `rayon` for generating new, deeper
  opening books.

- **Cross-Platform**: Available as a Rust crate, Python package, and WebAssembly module for
  seamless integration into a wide range of projects.

## Installation

This library can be added to a Cargo project by running the following command in your
project directory:

```shell
cargo add connect-four-ai
```

or by adding the following line to your Cargo.toml:

```shell
connect-four-ai = "1.0.0"
```

### Example Usage

This is a basic example of how to use the `Solver` to find the score of a position:

```rust
use connect_four_ai::{Solver, Position};

fn main() {
  // Creates a position from a sequence of 1-indexed moves
  let position = Position::from_moves("76461241141").unwrap();
  
  // Initialises and uses the Solver to calculate the exact score of the position
  let mut solver = Solver::new();
  let score = solver.solve(&position);
  
  println!("{score}");  // Output: -1
}
```

## License

This project is licensed under the **MIT License**. See the [`LICENSE`](https://github.com/benjaminrall/connect-four-ai/blob/main/LICENSE) file for details.
