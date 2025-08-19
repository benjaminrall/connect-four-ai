# Connect Four AI

[![Crates.io Version](https://img.shields.io/crates/v/connect-four-ai)](https://crates.io/crates/connect-four-ai)
[![PyPI Version](https://img.shields.io/pypi/v/connect-four-ai)](https://crates.io/crates/connect-four-ai)

A high-performance, perfect Connect Four solver written in Rust, with bindings for Python and WebAssembly.

// GIF of playing Connect Four

This repository contains a library which can solve any Connect Four position perfectly,
and therefore determine the optimal move to be played.
The core engine is implemented in Rust, using a highly optimised search algorithm
primarily based on the techniques described in this [blog](http://blog.gamesolver.org/).

## Table of Contents

- [Demos](#demos)
- [Key Features](#key-features)
- [Installation and Usage](#installation-and-usage)
  - [Rust](#rust)
  - [Python](#python)
  - [WebAssembly](#webassembly)
  - [Included Binaries](#included-binaries)
- [Development Setup](#development-setup)
- [License](#license)

## Demos

This repository contains two playable demos to showcase the engine's capabilities:
- **Web Demo**: A fully interactive demo built with WebAssembly. You can play against
  the AI, analyse positions, and see the solver in action. 
  - [View Source (`/web-demo`)](./web-demo)
- **Python Demo**: A simple Connect Four implementation built with Pygame.
  - [View Source (`/python-demo`)](./python-demo)

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

## Installation and Usage
The engine is available as a library for Rust, Python, and WebAssembly.

### Rust

The library is available on crates.io [here](https://crates.io/crates/connect-four-ai),
and can be added to a Cargo project by running the following command in your project
directory:

```shell
cargo add connect-four-ai
```

or by adding the following line to your Cargo.toml:

```shell
connect-four-ai = "0.1.1"
```

### Python

The library is available on PyPI [here](https://pypi.org/project/connect-four-ai),
and can be installed using the following command:

```shell
pip install connect-four-ai
```

### WebAssembly

// To be added

## Technical Details
<!-- 
- More in-depth details of how the Bitboard representation works
  and.. other stuff to do with the solver
-->

## Development Setup
<!-- 
- Very straightforward description of cloning the repo for development
- Description of adding a new feature in Rust in the core crate,
  then exposing it (if necessary) to Python and WASM through their respective crates
- Details of the included binaries for benchmarking and generating opening books
-->

To set up the project for development and contribution, first clone the repository:

```shell
git clone https://github.com/benjaminrall/connect-four-ai.git
cd connect-four-ai
```

The project is structured as a Cargo workspace in the [`./crates`](./crates) directory,
which contains three main components:
1. `core/`: The core Rust library.
2. `python/`: The Python bindings, built using `pyo3` and `maturin`.
3. `wasm/`: The WebAssembly bindings, built using `wasm-bindgen` and `wasm-pack`.

To add a new feature you would typically:
1. Implement the feature in the core Rust crate.
2. Expose the new functionality in the Python and WASM crates by adding the 
   necessary wrapper functions.
3. Re-build the packages to include the changes.

## License

This project is licensed under the **MIT License**. See the [`LICENSE`](https://github.com/benjaminrall/connect-four-ai/blob/main/LICENSE) file for details.
