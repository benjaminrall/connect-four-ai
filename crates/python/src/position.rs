//! A compact, computationally efficient bitboard representation of Connect 4 positions.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use connect_four_ai::Position;

/// Represents a Connect Four position compactly as a bitboard.
///
/// The standard, 6x7 Connect Four board can be represented unambiguously using 49 bits
/// in the following bit order:
///
/// ```comment
///   6 13 20 27 34 41 48
///  ---------------------
/// | 5 12 19 26 33 40 47 |
/// | 4 11 18 25 32 39 46 |
/// | 3 10 17 24 31 38 45 |
/// | 2  9 16 23 30 37 44 |
/// | 1  8 15 22 29 36 43 |
/// | 0  7 14 21 28 35 42 |
///  ---------------------
///```
///
/// The extra row of bits at the top identifies full columns and prevents
/// bits from overflowing into the next column. For computational
/// efficiency, positions are stored using two 64-bit unsigned integers:
/// one storing a mask of all occupied tiles, and the other storing a mask
/// of the current player's tiles.
#[pyclass(name="Position")]
#[derive(Copy, Clone, Debug)]
pub struct PyPosition(pub (crate) Position);

#[pymethods]
impl PyPosition {
    #[classattr]
    const WIDTH: usize = Position::WIDTH;
    #[classattr]
    const HEIGHT: usize = Position::HEIGHT;

    /// Creates a new position instance for the initial state of the game.
    #[new]
    fn new() -> PyResult<PyPosition> {
        Ok(PyPosition(Position::new()))
    }

    /// Parses a position from a string of 1-indexed moves.
    ///
    /// The input string should contain a sequence of columns played, indexed from 1.
    fn from_moves(moves: &str) -> PyResult<PyPosition> {
        Position::from_moves(moves)
            .map(PyPosition)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Parses a position from a string representation of the Connect Four board.
    ///
    /// The input string should contain exactly 42 characters from the set `['.', 'o', 'x']`,
    /// representing the board row by row from the top-left to the bottom-right. All other
    /// characters are ignored. 'x' is treated as the current player, and 'o' as the opponent.
    /// This method assumes that a correctly formatted board string is a valid game position.
    /// Invalid game positions will lead to undefined behaviour.
    fn from_board_string(board_string: &str) -> PyResult<PyPosition> {
        Position::from_board_string(board_string)
            .map(PyPosition)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Returns the Position's attributes formatted as a string.
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    /// A bitmask of the current player's tiles.
    #[getter]
    fn get_position(&self) -> u64 {
        self.0.position
    }

    /// A bitmask of all occupied tiles.
    #[getter]
    fn get_mask(&self) -> u64 {
        self.0.mask
    }

    /// Returns the unique key for the current position.
    ///
    /// This key is unique to each pair of horizontally symmetrical positions, as these
    /// positions will always have the same solution.
    fn get_key(&self) -> u64 {
        self.0.get_key()
    }

    /// Returns the number of moves played to reach the current position.
    fn get_moves(&self) -> usize {
        self.0.get_moves()
    }

    /// Indicates whether a given column is playable.
    ///
    /// # Arguments
    ///
    /// * `col`: 0-based index of a column.
    ///
    /// # Returns
    ///
    /// True if the column is playable, false if the column is already full.
    fn is_playable(&self, col: usize) -> bool {
        self.0.is_playable(col)
    }

    /// Indicates whether the current player wins by playing a given column.
    ///
    /// # Arguments
    ///
    /// * `col`: 0-based index of a playable column.
    ///
    /// # Returns
    ///
    /// True if the current player makes a 4-alignment by playing the column, false otherwise.
    fn is_winning_move(&self, col: usize) -> bool {
        self.0.is_winning_move(col)
    }

    /// Indicates whether the current player can win with their next move.
    fn can_win_next(&self) -> bool {
        self.0.can_win_next()
    }

    /// Plays a move in the given column.
    ///
    /// # Arguments
    ///
    /// * `col`: 0-based index of a playable column.
    fn play(&mut self, col: usize) {
        self.0.play(col)
    }

    /// Returns a mask for the possible moves the current player can make.
    fn possible(&self) -> u64 {
        self.0.possible()
    }

    /// Returns a mask for the possible non-losing moves the current player can make.
    fn possible_non_losing_moves(&self) -> u64 {
        self.0.possible_non_losing_moves()
    }

    /// Indicates whether the current position has been won by either player.
    fn is_won_position(&self) -> bool {
        self.0.is_won_position()
    }

    /// Returns a mask for the entirety of the given column.
    ///
    /// # Arguments
    ///
    /// * `col`: 0-based index of a column.
    ///
    /// # Returns
    ///
    /// A bitmask with a one in all cells of the column.
    #[staticmethod]
    fn column_mask(col: usize) -> u64 {
        Position::column_mask(col)
    }
}
