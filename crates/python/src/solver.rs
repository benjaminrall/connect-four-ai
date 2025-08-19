//! Provides the core solving logic for the Connect Four AI.

use std::path::Path;
use pyo3::prelude::*;
use connect_four_ai::Solver;
use crate::position::PyPosition;

/// A strong solver for finding the exact score of Connect Four positions.
///
/// This class implements a high-performance negamax search algorithm with several
/// optimisations, including:
/// - Alpha-beta pruning
/// - Score-based move ordering to prioritise stronger moves
/// - A transposition table to cache results of previously seen positions
/// - A binary search on the score for faster convergence
#[pyclass(name="Solver")]
#[derive(Debug)]
pub struct PySolver(Solver);

#[pymethods]
impl PySolver {
    /// Creates a new `Solver` instance, using the pre-packaged opening book.
    #[new]
    fn new() -> PySolver {
        PySolver(Solver::new())
    }

    /// A counter for the number of nodes explored in the last `solve` call.
    #[getter]
    fn get_explored_positions(&self) -> usize {
        self.0.explored_positions
    }

    /// Attempts to load an opening book from the given path.
    ///
    /// Returns whether the opening book was successfully loaded.
    fn load_opening_book(&mut self, path: &str) -> bool {
        self.0.load_opening_book(Path::new(path))
    }

    /// Resets the solver's state.
    fn reset(&mut self) {
        self.0.reset();
    }

    /// Solves a position to find its exact score.
    ///
    /// This function uses a binary search over the possible score range, repeatedly calling the
    /// negamax search with a null window to test if the score is above a certain value. This
    /// allows faster convergence to the true score.
    ///
    /// Assumes that the given position is valid and not won by either player.
    ///
    /// # Arguments
    ///
    /// * `position`: The board position to solve.
    ///
    /// # Returns
    /// The exact score of the position, which reflects the outcome of the game assuming that both
    /// players play perfectly. A position has:
    /// - A positive score if the current player will win. 1 if they win with their last move, 2 if
    ///   they win with their second to last move, ...
    /// - A null score if the game will end in a draw
    /// - A negative score if the current player will lose. -1 if the opponent wins with their last
    ///   move, -2 if the opponent wins with their second to last move, ...
    fn solve(&mut self, position: &PyPosition) -> i8 {
        self.0.solve(&position.0)
    }

    /// Calculates the scores for all possible next moves in the given position.
    ///
    /// Returns a fixed-size array where each index corresponds to a column, containing
    /// a score if a move in that column is possible and `None` if the column is full and
    /// the move is impossible.
    ///
    /// This array can be used to directly calculate the optimal move to play in a position.
    fn get_all_move_scores(&mut self, position: &PyPosition) -> Vec<Option<i8>> {
        self.0.get_all_move_scores(&position.0).to_vec()
    }
}