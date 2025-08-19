//! A high-performance implementation of a perfect Connect Four solver.
//!
//! This library provides functionality to compute the score and optimal move for any
//! given Connect Four position.

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
