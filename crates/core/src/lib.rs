//! A high-performance Rust library for a perfect Connect Four AI solver.
//!
//! This library provides functionality to compute the optimal move for any given
//! Connect Four board state. Since Connect Four is a solved game, this AI is capable of playing
//! perfectly.

mod board;
mod engine;

pub use engine::{
    Solver,
    TTFlag,
    TTEntry,
    TranspositionTable,
    MoveEntry,
    MoveSorter,
    OpeningBook,
    OpeningBookGenerator,
    Difficulty,
    AIPlayer
};
pub use board::{Position, PositionParsingError};
