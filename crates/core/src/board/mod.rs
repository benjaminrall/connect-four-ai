//! A compact, computationally efficient bitboard representation of Connect 4 positions.

mod position;
mod position_parsing_error;

pub use position::Position;
pub use position_parsing_error::PositionParsingError;
