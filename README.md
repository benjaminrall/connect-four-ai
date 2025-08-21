# Connect Four AI

[![Crates.io Version](https://img.shields.io/crates/v/connect-four-ai)](https://crates.io/crates/connect-four-ai)
[![PyPI Version](https://img.shields.io/pypi/v/connect-four-ai)](https://pypi.org/project/connect-four-ai)
[![NPM Version](https://img.shields.io/npm/v/connect-four-ai-wasm)](https://www.npmjs.com/package/connect-four-ai-wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/benjaminrall/connect-four-ai/blob/main/LICENSE)
[![docs.rs](https://img.shields.io/docsrs/connect-four-ai)](https://docs.rs/connect-four-ai)

A high-performance, perfect Connect Four solver written in Rust, with bindings for Python and WebAssembly.

![Connect Four GIF](https://github.com/user-attachments/assets/bb7dff1f-3a27-4f0a-b6ab-b46f19df6fd6)

This repository contains a library which can strongly solve any Connect Four position,
which allows it to also determine optimal moves.
The core engine is implemented in Rust, using a highly optimised search algorithm
primarily based on the techniques described in this [blog](http://blog.gamesolver.org/).

## Table of Contents

- [Demos](#demos)
- [Key Features](#key-features)
- [Performance](#performance)
  - [Test Methodology](#test-methodology)
  - [Benchmark Results](#benchmark-results)
- [Installation and Usage](#installation-and-usage)
  - [Rust](#rust)
  - [Python](#python)
  - [WebAssembly](#webassembly)
- [Development Setup](#development-setup)
  - [Building the Project](#building-the-project)
  - [Python Package](#python-package)
  - [WebAssembly Package](#webassembly-package)
  - [Included Tools](#included-tools)
- [Technical Details](#technical-details)
- [License](#license)

## Demos

This repository contains two playable demos to showcase the engine's capabilities:
- **Web Demo**: A fully interactive demo built with WebAssembly. You can play against
  the AI, analyse positions, and see the solver in action. 
  - [View Source (`./web-demo`)](./web-demo)
- **Python Demo**: A simple Connect Four implementation built with Pygame.
  - [View Source (`./python-demo`)](./python-demo)

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

## Performance

This engine is designed for high performance, capable of quickly solving
any Connect Four position strongly. Its speed was benchmarked against John Tromp's
[Fhourstones](https://en.wikipedia.org/wiki/Fhourstones) solver. A key difference
is that the Fhourstones benchmark is a weak solver, meaning it only determines
the win/loss/draw outcome, rather than the exact score. Despite this, it should
still provide a valuable baseline for comparing speed.

### Test Methodology

All benchmarks were run on the same machine using test data from this
[blog](http://blog.gamesolver.org/solving-connect-four/02-test-protocol/).
There are a total of 6000 positions, divided six datasets
of 1000 test cases each. 
Each dataset tests a different phase of the game, categorised by the number
of moves played (`n`) and the moves remaining until a forced conclusion (`r`).

| Test Set      | Game Depth (`n`) | Remaining Moves (`r`) |
|:--------------|:-----------------|:----------------------|
| end-easy      | `28 < n`         | `r < 14`              | 
| middle-easy   | `14 < n <= 28`   | `r < 14`              | 
| middle-medium | `14 < n <= 28`   | `14 <= r < 28`        |
| begin-easy    | `n <= 14`        | `r < 14`              | 
| begin-medium  | `n <= 14`        | `14 <= r < 28`        | 
| begin-hard    | `n <= 14`        | `28 <= r`             | 

### Benchmark Results

The table below shows the average time taken to solve a position, 
average number of positions explored, and search speed in thousands of
positions per second (kpos/s) for each solver on each test set. It also includes
results for using `connect-four-ai` with its default embedded opening book 
(of depth 8), and an extended opening book (of depth 12).

| Test Set          | Solver                          | Avg. Time   | Avg. Positions | Speed (kpos/s) |
|:------------------|:--------------------------------|:------------|:---------------|:---------------|
| **end-easy**      | Fhourstones                     | 4.27 µs     | **39.8**       | 9,331          |
|                   | connect-four-ai (no book)       | **3.32 µs** | 51             | **14,800**     |
| **middle-easy**   | Fhourstones                     | 137 µs      | 2,101          | **15,304**     |
|                   | connect-four-ai (no book)       | **32.2 µs** | **449**        | 13,952         |
| **middle-medium** | Fhourstones                     | **1.70 ms** | **28,725**     | **16,935**     |
|                   | connect-four-ai (no book)       | 2.87 ms     | 39,855         | 13,891         |
| **begin-easy**    | Fhourstones                     | 150 ms      | 2,456,184      | **16,325**     |
|                   | connect-four-ai (no book)       | 222 µs      | 3,295          | 14,824         |
|                   | connect-four-ai (depth 8 book)  | 159 µs      | 2,294          | 14,404         |
|                   | connect-four-ai (depth 12 book) | **42 µs**   | **619**        | 14,706         |
| **begin-medium**  | Fhourstones                     | 80.6 ms     | 1,296,896      | 16,088         |
|                   | connect-four-ai (no book)       | 87.3 ms     | 1,191,372      | 13,634         |
|                   | connect-four-ai (depth 8 book)  | 49.0 ms     | 631,766        | 12,895         |
|                   | connect-four-ai (depth 12 book) | **7.44 ms** | **95,156**     | 12,790         |
| **begin-hard**    | Fhourstones                     | 5.58 s      | 93,425,554     | **16,743**     |
|                   | connect-four-ai (no book)       | 5.09 s      | 64,186,798     | 12,618         |
|                   | connect-four-ai (depth 8 book)  | 67.3 ms     | 834,100        | 12,387         |
|                   | connect-four-ai (depth 12 book) | **1.40 ms** | **17,631**     | 12,623         |

The results clearly demonstrate the impact of the opening book. On the `begin-hard`
test set, a depth 12 book reduces the runtime by a factor of 3,600x by solving
the most complex opening positions instantly. Even without the book, the engine
is often faster than Fhourstones due to its efficiency in exploring significantly
fewer positions. While Fhourstones maintains a higher raw speed, this engine's
speed is still highly competitive given that it performs the more complex task
of strong solving.

## Installation and Usage
The engine is available as a library for Rust, Python, and WebAssembly.
The core Rust library provides access to all the engine's components, 
while the Python and WASM bindings expose a simplified API, only including 
the `Position`, `Solver`, and `AIPlayer`.

### Rust

The core library is available on crates.io [here](https://crates.io/crates/connect-four-ai),
and can be added to a Cargo project by running the following command in your project
directory:

```shell
cargo add connect-four-ai
```

or by adding the following line to your Cargo.toml:

```shell
connect-four-ai = "0.1.2"
```

#### Example

This is a basic example of how to use the `Solver` to find the score of a position in Rust:

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

### Python

The library is available on PyPI [here](https://pypi.org/project/connect-four-ai),
and can be installed using the following command:

```shell
pip install connect-four-ai
```

#### Example
This is a basic example of how to use the `Solver` to find the score of a position in Python:

```python
from connect_four_ai import Solver, Position

# Creates a position from a sequence of 1-indexed moves
position = Position.from_moves("76461241141")

# Initialises and uses the Solver to calculate the exact score of the position
solver = Solver()
score = solver.solve(position)

print(score)    # Output: -1
```

### WebAssembly

The library is available on npm [here](https://www.npmjs.com/package/connect-four-ai),
and can be installed using the following command:

```shell
npm install connect-four-ai-wasm
```

#### Example

This is a basic example of how to use the `Solver` to find the score of a position in JavaScript:

```javascript
import init, { Solver, Position } from "connect-four-ai-wasm";

async function run() {
  // Initialises the WASM module
  await init();

  // Creates a position from a sequence of 1-indexed moves
  let position = Position.fromMoves("76461241141");

  // Initialises and uses the Solver to calculate the exact score of the position
  let solver = new Solver();
  let score = solver.solve(position);

  console.log(score); // Output: -1
}

run();
```

## Development Setup

To set up the project for development and contribution, first clone the repository:

```shell
git clone https://github.com/benjaminrall/connect-four-ai.git
cd connect-four-ai
```

The project is structured as a Cargo workspace, with all the code located in the [`./crates`](./crates) directory.
This workspace contains three main components:
1. `core/`: The core Rust library containing all the engine's logic.
2. `python/`: The Python bindings, built using `pyo3` and `maturin`.
3. `wasm/`: The WebAssembly bindings, built using `wasm-bindgen` and `wasm-pack`.

### Building the Project

All crates in the workspace can be built by using the
following command from the `crates` directory:

```shell
cd crates
cargo build --release
```

Note that this command will not produce the final Python or WebAssembly packages.
To build those, you must use their specific build tools as described below.

### Python Package

The Python package is managed with `maturin`.
To set up a virtual environment and build the package for testing:

1. Navigate to the Python crate directory:
   ```shell
   cd crates/python
   ```
2. Create and activate a virtual environment:
   ```shell
   python -m venv .venv
   source .venv/bin/activate  # On Windows, use `.venv\Scripts\activate`
   ```
3. Install the package in editable mode:
   ```shell
   pip install -e .
   ```

### WebAssembly Package

The WebAssembly package is built with `wasm-pack`.
To compile and test the package:

1. Navigate to the WASM crate directory:
   ```shell
   cd crates/wasm
   ```
2. Install `wasm-pack` if you haven't already:
   ```shell
   cargo install wasm-pack
   ```
3. Build the package for the web:
   ```shell
   wasm-pack build --target web
   ```
4. Edit and host the example `index.html` file to test the package,
   for example using Python's built-in HTTP server:
   ```shell
   python -m http.server
   ```

### Included Tools

The repository includes command-line tools for benchmarking and book generation,
which can be run from the root of the workspace.

- **Benchmarking**: Evaluates the performance and accuracy of the `Solver` by
  running it on a set of predefined test positions and their scores.
  ```shell
  # Runs the default solver against the set of test positions found in the file `test-data/begin-hard`
  cargo run --release --bin benchmark -- test-data/begin-hard
  ```
- **Book Generator**: To generate a new opening book of a specified depth.
  ```shell
  # Generates a book of depth 10 and saves it to `book.bin`
  cargo run --release --bin generate_book -- 10 book.bin
  ```

## Technical Details

Below are more in-depth details of the core algorithm and optimisations
that power the engine.

### Solver

The main `Solver` class uses the negamax algorithm with many optimisations,
detailed below.

#### Negamax Algorithm

Negamax is a variant of the minimax algorithm used for zero-sum two-player games. 
It leverages the fact that a score from your opponent's point of view is the 
negated score of the same position from your point of view. This allows the 
implementation to be greatly simplified, as you don't require separate logic 
for the maximising and minimising players and can use a single recursive function.

#### Alpha-beta Pruning

Alpha-beta pruning is a method which can decrease the number of nodes the
negamax algorithm needs to evaluate in the game tree. 
It introduces a lower and upper bound (alpha and beta) for the score of a node 
at a given depth, which allow the search can be cut off 
(thereby pruning portions of the game tree) when negamax
encounters a node outside this range.

#### Move Ordering

Move ordering is an optimisation for alpha-beta pruning that attempts to guess
and prioritise moves which are more likely to yield the node's score. The result
of good guesses is earlier and more frequent alpha/beta cut-offs, allowing
more game tree branches to be pruned. For this implementation,
moves are first ordered by the number of potential winning positions they create. 
Any ties are broken by a pre-defined sequence that prioritises columns closer 
to the centre.

#### Transposition Table

When exploring the game tree, it's common to reach the same position
through different sequences of moves. To avoid re-computing the score for
positions that have already been explored, the engine uses a transposition table,
which acts as a large cache of previous search results. The table itself is a
large array, with each entry containing key information such as a position's score, 
search depth, and whether the score is an exact value or a bound. Additionally,
the table is optimised by only storing a partial key for verification. This
technique is justified by the Chinese Remainder Theorem, and allows each entry to
be packed into just 8 bytes, which is highly memory-efficient and allows millions
of positions to be cached.

#### Binary Search and Null Windows

Instead of performing a single, wide search for the score, the solver pinpoints
the exact value using a binary search over the possible score range. At each step, 
it uses a null window search to test if the true score is better than
the current guess (`mid`). A null window search is a highly optimised search that
sets the search bounds to a minimal range `[mid, mid + 1]`. This allows for more
aggressive pruning and a faster answer to refine the search window.

### Bitboard Representation

One of the most important optimisations is representing Connect Four positions
using a bitboard. The standard 6x7 board can be represented by 49 bits as follows:

```comment
   6 13 20 27 34 41 48
  ---------------------
 | 5 12 19 26 33 40 47 |
 | 4 11 18 25 32 39 46 |
 | 3 10 17 24 31 38 45 |
 | 2  9 16 23 30 37 44 |
 | 1  8 15 22 29 36 43 |
 | 0  7 14 21 28 35 42 |
  ---------------------
```

Each column has an extra 'padding' bit at the top to prevent bitwise operations from
overflowing into the next column. For computational efficiency, positions are stored in 
practice using two `u64` values: one for a mask of all occupied tiles (`mask`), and one for a
mask of just the current player's tiles (`position`).

This representation allows game logic, such as playing a move or detecting a win, to be
performed with incredibly fast bitwise operations. This is orders of magnitudes faster
than iterating over a 2D array and speeds up every aspect of the engine.

Additionally, these two integers can be summed to generate a unique key for accessing
the transposition table and opening book. Since Connect Four positions are horizontally
symmetrical, both the key for a position and its horizontal mirror are computed, and then
the smaller of the two is used, effectively halving the number of positions that need to be stored.

### Opening Book

The engine also uses an opening book, which is a pre-computed database that stores the exact scores
for all game positions up to a certain depth. When the solver encounters a position that exists
in the book, it can return the stored score instantly, bypassing the need to carry out the full
search. The book is implemented as a simple hash map, mapping a position's unique, symmetry-aware key
to its score. A default book containing all positions up to 8 moves deep is serialised and embedded
directly into the library's binary, which provides a massive improvement to performance on early
game positions without requiring users to load any external files.

For advanced use, the repository also includes a tool for generating new opening books. It performs
an exhaustive, parallelised breadth-first search of the game tree up to a given depth, using the
solver to evaluate each position.

## License

This project is licensed under the **MIT License**. See the [`LICENSE`](https://github.com/benjaminrall/connect-four-ai/blob/main/LICENSE) file for details.
