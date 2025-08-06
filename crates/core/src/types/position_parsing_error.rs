use std::error::Error;
use std::fmt::{Display, Formatter};

/// An enum for errors that can occur when parsing Connect Four positions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionParsingError {
    /// The board string has an invalid number of characters.
    InvalidBoardStringLength { actual: usize, expected: usize },

    /// The move sequence contains an invalid (non-numeric) character.
    InvalidCharacter { character: char, index: usize },

    /// The move sequence contains an invalid, out of range column.
    InvalidColumn { column: usize, index: usize },

    /// The move sequence contains an invalid move as a result of a full column.
    InvalidFullColumnMove { column: usize, index: usize },

    /// The move sequence contains an invalid move that results in a winning position.
    InvalidWinningMove { column: usize, index: usize },
}

impl Display for PositionParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PositionParsingError::InvalidBoardStringLength { actual, expected } =>
                write!(
                    f,
                    "board string had an invalid number of characters: found {}, expected {}",
                    actual,
                    expected
                ),
            PositionParsingError::InvalidCharacter { character, index } =>
                write!(f, "invalid character found in move sequence: {} at index {}", character, index),
            PositionParsingError::InvalidColumn { column, index } =>
                write!(f, "invalid column index found in move sequence: {} at index {}", column, index),
            PositionParsingError::InvalidFullColumnMove { column, index } =>
                write!(f, "invalid move found in move sequence: column {} is full at index {}", column, index),
            PositionParsingError::InvalidWinningMove { column, index } =>
                write!(f, "invalid move found in move sequence: column {} results in a win at index {}", column, index),
        }
    }
}

impl Error for PositionParsingError {}