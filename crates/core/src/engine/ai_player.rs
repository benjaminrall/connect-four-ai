use std::cmp::Ordering;
use std::path::Path;
use rand::distr::weighted::WeightedIndex;
use rand::{rng};
use rand::distr::Distribution;
use crate::{Position, Solver};

/// An enum to represent the difficulty of an AI player.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Impossible,
}

impl Difficulty {
    /// Returns the softmax temperature associated with a difficulty level.
    pub fn temperature(&self) -> f64 {
        match self {
            Difficulty::Easy => 0.25,
            Difficulty::Medium => 0.1,
            Difficulty::Hard => 0.025,
            Difficulty::Impossible => 0.,
        }
    }
}

/// An AI player that uses a solver to determine the best move to play in a Connect Four position.
///
/// The player's skill level can be configured using the `Difficulty` enum, which adjusts the
/// move selection strategy.
pub struct AIPlayer {
    solver: Solver,
    difficulty: Difficulty,
}

impl AIPlayer {
    /// Creates a new AI player with a default solver and specified difficulty.
    pub fn new(difficulty: Difficulty) -> AIPlayer {
        AIPlayer {
            solver: Solver::new(),
            difficulty,
        }
    }

    /// Attempts to load an opening book from the given path for the AI player's solver.
    ///
    /// Returns whether the opening book was successfully loaded.
    pub fn load_opening_book(&mut self, path: &Path) -> bool {
        self.solver.load_opening_book(path)
    }

    /// Resets the AI player's solver.
    pub fn reset(&mut self) {
        self.solver.reset();
    }

    /// Solves a position to find its exact score using the AI player's solver.
    pub fn solve(&mut self, position: &Position) -> i8 {
        self.solver.solve(position)
    }

    /// Calculates the scores for all possible next moves in the given position using the
    /// AI player's solver.
    pub fn get_all_move_scores(&mut self, position: &Position) -> [Option<i8>; Position::WIDTH] {
        self.solver.get_all_move_scores(position)
    }

    /// Calculates the AI player's selected move for the given position.
    pub fn get_move(&mut self, position: &Position) -> Option<usize> {
        let move_scores = self.solver.get_all_move_scores(position);
        self.select_move(position, &move_scores)
    }

    /// Selects a move from a fixed-size array of scores using a Softmax distribution with a
    /// temperature defined by the AI player's difficulty. Temperature values <= 0 will
    /// result in greedy selection (always picking the best move).
    ///
    /// Returns an `Option<usize>` containing the column index of the selected move, or `None`
    /// if no moves are possible.
    pub fn select_move(&self, position: &Position, scores: &[Option<i8>; Position::WIDTH]) -> Option<usize> {
        let normalised_scores = Self::normalise_scores(position, scores);
        let possible_moves: Vec<(usize, f64)> = normalised_scores
            .iter()
            .enumerate()
            .filter_map(|(col_index, score_option)| score_option.map(|score| (col_index, score)))
            .collect();

        // Returns `None` if no possible moves could be found
        if possible_moves.is_empty() {
            return None
        }

        // Greedily selects the optimal move if the temperatures is zero or less
        let temperature = self.difficulty.temperature();
        if temperature <= 0.0 {
            return possible_moves
                .iter()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(index, _)| *index);
        }

        // Otherwise, samples from the scores using a softmax distribution
        let weights: Vec<f64> = possible_moves
            .iter()
            .map(|(_, score)| (*score as f64 / temperature).exp())
            .collect();

        let dist = match WeightedIndex::new(&weights) {
            Ok(weighted_index) => weighted_index,
            Err(_) => return None,
        };

        let mut rng = rng();
        let selected_index = dist.sample(&mut rng);

        Some(possible_moves[selected_index].0)
    }

    /// Normalises scores from the given position to lie in the range -1 to 1, scaled by the maximum
    /// possible score.
    pub fn normalise_scores(position: &Position, scores: &[Option<i8>; Position::WIDTH]) -> [Option<f64>; Position::WIDTH] {
        let mut normalised_scores = [None; Position::WIDTH];
        let max_possible_score = ((Position::BOARD_SIZE + 1 - position.get_moves()) as i8 / 2) as f64;

        if max_possible_score <= 0.0 {
            return normalised_scores;
        }

        for i in 0..Position::WIDTH {
            if let Some(score) = scores[i] {
                normalised_scores[i] = Some(score as f64 / max_possible_score);
            }
        }
        normalised_scores
    }
}