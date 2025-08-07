//! A high-performance Rust library for a perfect Connect Four AI solver.
//!
//! This library provides functionality to compute the optimal move for any given
//! Connect Four board state. Since Connect Four is a solved game, this AI plays perfectly.

mod types;
mod solver;
mod utils;

pub use types::{Position, PositionParsingError, TranspositionTable, MoveEntry, MoveSorter, TTEntry, TTFlag};
pub use solver::Solver;
pub use utils::*;