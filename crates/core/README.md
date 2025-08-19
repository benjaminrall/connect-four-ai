# Connect Four AI

[![Crates.io Version](https://img.shields.io/crates/v/connect-four-ai)](https://crates.io/crates/connect-four-ai)
[![PyPI Version](https://img.shields.io/pypi/v/connect-four-ai)](https://crates.io/crates/connect-four-ai)

A high-performance, perfect Connect Four solver written in Rust, with bindings for Python and WebAssembly.

// GIF of playing Connect Four

This repository contains a library which can solve any Connect Four position perfectly,
and therefore determine the optimal move to be played.
The core engine is implemented in Rust, using a highly optimised search algorithm
primarily based on the techniques described in this [blog](http://blog.gamesolver.org/).

## Key Features

- **Perfect Solver**: Implements an optimised [negamax search](https://en.wikipedia.org/wiki/Negamax),
  which utilises alpha-beta pruning and a transposition table to quickly converge on
  exact game outcomes.

- **Bitboard Representation**: Uses a compact and efficient bitboard representation for
  game positions, allowing for fast move generation and evaluation.

- **Embedded Opening Book**: Includes a pre-generated opening book of depth 8, which is
  embedded directly into the binary for instant lookups of early-game solutions.

- **Parallel Book Generator**: A tool built with `rayon` for generating new, deeper
  opening books.

- **Cross-Platform**: Available as a Rust crate, Python package, and WebAssembly module for
  seamless integration into a wide range of projects.

## Usage
<!-- 
- Simple installation & usage details of the library for Rust, Python, and WASM

-->


## Technical Details
<!-- 

-->

## Development Setup
<!-- 

-->

## License

This project is licensed under the **MIT License**. See the [`LICENSE`](./LICENSE) file for details.
