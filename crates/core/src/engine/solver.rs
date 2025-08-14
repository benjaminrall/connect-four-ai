//! Provides the core solving logic for the Connect Four AI.

use crate::{MoveSorter, OpeningBook, Position, TTFlag, TranspositionTable};
use std::path::Path;

// This line embeds a book file directly into the program's binary
// The path is relative to the current source file (solver.rs)
const OPENING_BOOK_BYTES: &[u8] = include_bytes!("books/default-book.bin");

/// A strong solver for finding the exact score of Connect Four positions.
///
/// This struct implements a high-performance negamax search algorithm with several
/// optimisations, including:
/// - Alpha-beta pruning
/// - Score-based move ordering to prioritise stronger moves
/// - A transposition table to cache results of previously seen positions
/// - A binary search on the score for faster convergence
#[derive(Debug)]
pub struct Solver {
    /// A counter for the number of nodes explored since the last reset.
    pub explored_positions: usize,

    /// The transposition table used for caching search results.
    pub transposition_table: TranspositionTable,

    /// The opening book for instant lookups of early-game positions.
    pub opening_book: Option<OpeningBook>,
}


impl Solver {
    /// A pre-sorted list of columns to check, starting from the centre column.
    const COLUMNS: [usize; Position::WIDTH] = const {
        let mut columns = [0; Position::WIDTH];
        let mut i = 0;
        while i < Position::WIDTH {
            columns[i] = (Position::WIDTH as i32 / 2 + (1 - 2 * (i as i32 % 2)) * (i as i32 + 1) / 2) as usize;
            i += 1;
        }
        columns
    };

    /// Creates a new `Solver` instance, using the pre-packaged opening book.
    pub fn new() -> Solver {
        Self::default()
    }

    /// Creates a new `Solver` instance which is empty (without an opening book).
    pub fn empty() -> Solver {
        Solver {
            explored_positions: 0,
            transposition_table: TranspositionTable::new(),
            opening_book: None
        }
    }

    /// Attempts to load an opening book from the given path.
    ///
    /// Returns whether the opening book was successfully loaded.
    pub fn load_opening_book(&mut self, path: &Path) -> bool {
        self.opening_book = OpeningBook::load(path).ok();
        self.opening_book.is_some()
    }

    /// Resets the solver's state.
    pub fn reset(&mut self) {
        self.explored_positions = 0;
        self.transposition_table.reset();
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
    pub fn solve(&mut self, position: &Position) -> i8 {
        self.explored_positions = 0;

        // Before starting the search, checks if the answer is in the opening book
        if let Some(score) = self.opening_book.as_ref().and_then(|book| book.get(position)) {
            return score;
        }

        // Initial search window is the widest possible score range
        let mut min = -((Position::BOARD_SIZE - position.get_moves()) as i8) / 2;
        let mut max = (Position::BOARD_SIZE + 1 - position.get_moves()) as i8 / 2;

        while min < max {
            // Binary search for the true score
            let mut mid = min + (max - min) / 2;
            if mid <= 0 && min / 2 < mid {
                mid = min / 2
            } else if mid >= 0 && max / 2 > mid {
                mid = max / 2
            }

            // Performs a null-window search to test if the score is greater than the midpoint
            let score = self.negamax(position, (Position::BOARD_SIZE - position.get_moves()) as u8, mid, mid + 1);

            // Adjusts the search window based on the result
            if score <= mid {
                max = score
            } else {
                min = score
            }
        }

        min
    }

    /// Calculates the scores for all possible next moves in the given position.
    ///
    /// Returns a fixed-size array where each index corresponds to a column.
    /// - `Some(score)`: The score if a move in that column is possible.
    /// - `None`: If the column is full and the move is impossible.
    ///
    /// This array can be used to directly calculate the optimal move to play in a position.
    pub fn get_all_move_scores(&mut self, position: &Position) -> [Option<i8>; Position::WIDTH] {
        let mut scores = [None; Position::WIDTH];
        let depth = (Position::BOARD_SIZE - position.get_moves()) as u8;

        // If the game is won or the position is full, no moves are possible
        if position.is_won_position() || depth == 0 {
            return scores;
        }

        // Gets a bitmask of all possible moves in the position
        let moves = position.possible();

        // Loops through all playable columns, calculating and storing their scores
        for &column in Self::COLUMNS.iter() {
            if moves & Position::column_mask(column) == 0 {
                continue;
            }

            if position.is_winning_move(column) {
                scores[column] = Some((Position::BOARD_SIZE - position.get_moves() + 1) as i8 / 2);
                continue;
            }

            let mut new_position = *position;
            new_position.play(column);
            scores[column] = Some(-self.solve(&new_position));
        }

        scores
    }

    /// The core negamax search function with alpha-beta pruning.
    pub fn negamax(&mut self, position: &Position, depth: u8, mut alpha: i8, mut beta: i8) -> i8 {
        self.explored_positions += 1;

        // Checks for a drawn game
        if depth == 0 {
            return 0;
        }

        // Checks if the current player can win the game
        for i in 0..Position::WIDTH {
            if position.is_playable(i) && position.is_winning_move(i) {
                return (Position::BOARD_SIZE + 1 - position.get_moves()) as i8 / 2
            }
        }

        // Transposition table look-up
        let original_alpha = alpha;
        let key = position.get_key();
        if let Some(entry) = self.transposition_table.get(key) {
            if entry.depth >= depth {
                match entry.flag {
                    TTFlag::Exact => return entry.value,
                    TTFlag::LowerBound if entry.value >= beta => return entry.value,
                    TTFlag::UpperBound if entry.value <= alpha => return entry.value,
                    _ => {} // Can't use the entry, so continue the search.
                }
            }
        }


        // Move generation and pruning
        let possible_moves = position.possible_non_losing_moves();
        if possible_moves == 0 {
            // If there are no possible non-losing moves, then the opponent is guaranteed to win
            return -((Position::BOARD_SIZE - position.get_moves()) as i8) / 2;
        }

        // Tightens the lower bound as the opponent cannot win next move
        let min = -((Position::BOARD_SIZE - position.get_moves()) as i8 - 2) / 2;
        if alpha < min {
            if min >= beta { return min }
            alpha = min;
        }

        // Tightens the upper bound as we cannot win immediately
        let max = ((Position::BOARD_SIZE - position.get_moves()) as i8 - 1) / 2;
        if beta > max {
            if alpha >= max { return max }
            beta = max;
        }

        // Scores and sorts possible moves to explore the best ones first
        let mut moves = MoveSorter::new();
        for &column in Self::COLUMNS.iter().rev() {
            let move_bit = possible_moves & Position::column_mask(column);
            if move_bit > 0 {
                moves.add(column, position.score_move(move_bit))
            }
        }

        // Computes the scores of all possible next moves, keeping the best
        for column in moves {
            let mut new_position = *position;
            new_position.play(column);
            let score = -self.negamax(&new_position, depth - 1, -beta, -alpha);
            if score > alpha {
                alpha = score;
            }

            // Stops searching if a score is found outside the search window
            if alpha >= beta {
                break;
            }
        }

        // Stores the result of this search to the transposition table
        let flag = if alpha <= original_alpha {
            TTFlag::UpperBound
        } else if alpha >= beta {
            TTFlag::LowerBound
        } else {
            TTFlag::Exact
        };
        self.transposition_table.put(key, alpha, flag, depth);

        alpha
    }
}

/// Default constructor for the `Solver` struct.
impl Default for Solver {
    fn default() -> Solver {
        Solver {
            explored_positions: 0,
            transposition_table: TranspositionTable::new(),
            opening_book: OpeningBook::from_static_bytes(OPENING_BOOK_BYTES).ok()
        }
    }
}