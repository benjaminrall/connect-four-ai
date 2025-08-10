//! Provides the core solving logic for the Connect Four AI.

use crate::{MoveSorter, OpeningBook, Position, TTFlag, TranspositionTable};
use std::path::Path;

// This line embeds the generated book file directly into your program's binary.
// The path is relative to the current source file (solver.rs).
const OPENING_BOOK_BYTES: &[u8] = include_bytes!("books/default-book.bin");

/// A strong solver for finding the exact score of a Connect Four position.
///
/// This struct implements a high-performance negamax search algorithm with several
/// optimisations, including:
/// - Alpha-beta pruning
/// - Score-based move ordering to prioritise stronger moves
/// - A transposition table to cache results of previously seen positions
/// - A binary search on the score for faster convergence
#[derive(Debug)]
pub struct Solver {
    /// A counter for the number of nodes explored in the last search
    pub explored_positions: usize,

    /// The transposition table used for caching search results
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

    /// Attempts to load an opening book to be used by the solver.
    /// Returns whether the opening book was successfully loaded.
    pub fn load_opening_book(&mut self, path: &Path) -> bool{
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
        // Before starting the search, checks if the answer is in the opening book
        if let Some(book) = &self.opening_book {
            if let Some(score) = book.get(position) {
                // println!("{:?} {} {}", position, position.get_key(), position.get_mirrored_key());
                return score;
            }
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

    /// The core negamax search function with alpha-beta pruning.
    fn negamax(&mut self, position: &Position, depth: u8, mut alpha: i8, mut beta: i8) -> i8 {
        self.explored_positions += 1;

        // Checks for a drawn game
        if depth == 0 {
            return 0;
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
            if min >= beta { return min}
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