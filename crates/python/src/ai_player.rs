//! Provides the logic for an AI player which utilises a solver to select which move to play
//! in a given Connect Four position.
//!
//! The AI's skill is determined by the `Difficulty` enum, which controls the temperature of
//! a softmax distribution over the scores of each move in the position. This allows for a range
//! of behaviours, from a more random 'Easy' player to a perfect, greedy 'Impossible' player.

use std::path::Path;
use connect_four_ai::{AIPlayer, Difficulty, Position};
use pyo3::prelude::*;
use crate::position::PyPosition;

/// An enum to represent the difficulty of an AI player.
#[pyclass(name="Difficulty")]
#[derive(Copy, Clone, Debug)]
pub struct PyDifficulty(Difficulty);

#[pymethods]
impl PyDifficulty {
    #[classattr]
    const EASY: Self = Self(Difficulty::Easy);
    #[classattr]
    const MEDIUM: Self = Self(Difficulty::Medium);
    #[classattr]
    const HARD: Self = Self(Difficulty::Hard);
    #[classattr]
    const IMPOSSIBLE: Self = Self(Difficulty::Impossible);
}

/// An AI player that uses a solver to determine the best move to play in a Connect Four position.
///
/// The player's skill level can be configured using the `Difficulty` enum, which adjusts the
/// move selection strategy.
#[pyclass(name="AIPlayer")]
#[derive(Debug)]
pub struct PyAIPlayer(AIPlayer);

#[pymethods]
impl PyAIPlayer {
    /// Creates a new AI player with a default solver and specified difficulty.
    #[new]
    #[pyo3(signature=(difficulty=PyDifficulty::IMPOSSIBLE))]
    fn new(difficulty: PyDifficulty) -> PyAIPlayer {
        PyAIPlayer(AIPlayer::new(difficulty.0))
    }

    /// Attempts to load an opening book from the given path for the AI player's solver.
    ///
    /// Returns whether the opening book was successfully loaded.
    fn load_opening_book(&mut self, path: &str) -> bool {
        self.0.load_opening_book(Path::new(path))
    }

    /// Resets the AI player's solver.
    fn reset(&mut self) {
        self.0.reset();
    }

    /// Solves a position to find its exact score using the AI player's solver.
    fn solve(&mut self, position: &PyPosition) -> i8 {
        self.0.solve(&position.0)
    }

    /// Calculates the scores for all possible next moves in the given position using the
    /// AI player's solver.
    pub fn get_all_move_scores(&mut self, position: &PyPosition) -> Vec<Option<i8>> {
        self.0.get_all_move_scores(&position.0).to_vec()
    }

    /// Solves and selects the AI player's move for the given position.
    pub fn get_move(&mut self, position: &PyPosition) -> Option<usize> {
        self.0.get_move(&position.0)
    }

    /// Selects a move from an array of scores using a Softmax distribution with a
    /// temperature defined by the AI player's difficulty. Temperature values <= 0 will
    /// result in greedy selection (always picking the best move).
    ///
    /// Returns the column index of the selected move or `None` if no moves are possible.
    pub fn select_move(&self, position: &PyPosition, scores: Vec<Option<i8>>) -> Option<usize> {
        let scores_array: [Option<i8>; Position::WIDTH] = scores
            .clone()
            .try_into()
            .unwrap_or_else(|_|
                panic!("Scores vector must contain exactly {} elements.", Position::WIDTH)
            );

        self.0.select_move(&position.0, &scores_array)
    }
}