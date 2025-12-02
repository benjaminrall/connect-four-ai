//! Provides the core solving logic for the Connect Four AI.

use crate::{MoveSorter, OpeningBook, Position, TTFlag, TranspositionTable};
use std::path::Path;

// This line embeds a book file directly into the program's binary
// The path is relative to the current source file (solver.rs)
const OPENING_BOOK_BYTES: &[u8] = include_bytes!("books/default-book.bin");

#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    if b {
        #[cold]
        fn cold_path() {}
        cold_path();
    }
    b
}

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
    /// A counter for the number of nodes explored in the last `solve` call.
    pub explored_positions: usize,

    /// The transposition table used for caching search results.
    pub transposition_table: TranspositionTable,

    /// The opening book for instant lookups of early-game positions.
    pub opening_book: Option<OpeningBook>,
}


impl Solver {
    /// A pre-sorted list of columns to check, starting from the centre column.
    pub const COLUMNS: [usize; Position::WIDTH] = const {
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
    #[inline(always)]
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

        // Pre-compute winning positions for this player and early exit if they already win.
        let winning_spots = position.winning_positions();
        let moves = position.get_moves();
        let depth = (Position::BOARD_SIZE - moves) as u8;
        if winning_spots & position.possible() > 0 {
            return (Position::BOARD_SIZE + 1 - moves) as i8 / 2;
        }

        // Initial search window is the widest possible score range
        let mut min = -(depth as i8) / 2;
        let mut max = (Position::BOARD_SIZE + 1 - moves) as i8 / 2;

        while min < max {
            // Binary search for the true score
            let mut mid = min + (max - min) / 2;
            if mid <= 0 && min / 2 < mid {
                mid = min / 2
            } else if mid >= 0 && max / 2 > mid {
                mid = max / 2
            }

            // Performs a null-window search to test if the score is greater than the midpoint
            let score = self.negamax_wrapper(*position, mid, mid + 1, winning_spots);

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
        let moves = position.get_moves();
        let depth = (Position::BOARD_SIZE - moves) as u8;

        // If the game is won or the position is full, no moves are possible
        if position.is_won_position() || depth == 0 {
            return scores;
        }

        // Loops through all columns, calculating and storing their scores if they're playable
        for &column in Self::COLUMNS.iter() {
            if !position.is_playable(column) {
                continue;
            }

            if position.is_winning_move(column) {
                scores[column] = Some((Position::BOARD_SIZE - moves + 1) as i8 / 2);
            } else {
                let mut new_position = *position;
                new_position.play(column);
                scores[column] = Some(-self.solve(&new_position));
            }
        }

        scores
    }

    #[inline(always)]
    pub fn generate_moves(position: Position, possible_moves_mask: u64) -> MoveSorter {
        let mut moves = MoveSorter::new();

        if possible_moves_mask.count_ones() == 1 {
            let column = (possible_moves_mask.trailing_zeros() / 7) as usize;
            moves.add(column, 0);
        } else {
            for &column in Self::COLUMNS.iter().rev() {
                let move_bit = possible_moves_mask & Position::column_mask(column);
                if move_bit > 0 {
                    moves.add(column, position.score_move(move_bit))
                }
            }
        }
        moves
    }

    #[inline(always)]
    fn negamax_wrapper(
        &mut self,
        position: Position,
        alpha: i8,
        beta: i8,
        next_winning_spots: u64,
    ) -> i8 {

        let depth = (Position::BOARD_SIZE - position.get_moves()) as u8;

        // Checks for a drawn game
        if unlikely(depth == 0) {
            return 0;
        }

        // Use the mask from the parent to detect an immediate win without recomputing.
        if unlikely(next_winning_spots & position.possible() > 0) {
            return (depth as i8 + 1) / 2;
        }

        self.negamax(position, alpha, beta)
    }

    /// The core negamax search function with alpha-beta pruning.
    fn negamax(
        &mut self,
        position: Position,
        mut alpha: i8,
        mut beta: i8,
    ) -> i8 {
        self.explored_positions += 1;

        let (key, tt_index) = {
            let key = position.get_key();
            let tt_index = self.transposition_table.index(key);
            self.transposition_table.prefetch_index(tt_index);
            (key, tt_index)
        };

        // Calculate opponent threats once and reuse the mask for the children.
        let opponent_winning_spots = position.opponent_winning_positions();
        let depth = (Position::BOARD_SIZE - position.get_moves()) as u8;
        let mut possible_moves = position.possible();

        // Move generation and pruning
        let forced_moves = possible_moves & opponent_winning_spots;
        if unlikely(forced_moves > 0) {
            if forced_moves & (forced_moves - 1) > 0 {
                return -(depth as i8) / 2;
            }
            possible_moves = forced_moves;
        }

        possible_moves &= !(opponent_winning_spots >> 1);
        if unlikely(possible_moves == 0) {
            // If there are no possible non-losing moves, then the opponent is guaranteed to win
            return -(depth as i8) / 2;
        }

        // Tightens the lower bound as the opponent cannot win next move
        let min = -((depth as i8) - 2) / 2;
        if alpha < min {
            if min >= beta { return min }
            alpha = min;
        }

        // Tightens the upper bound as we cannot win immediately
        let max = ((depth as i8) - 1) / 2;
        if beta > max {
            if alpha >= max { return max }
            beta = max;
        }

        // Transposition table look-up
        let original_alpha = alpha;
        let mut tt_best_move = 7;
        if let Some(entry) = self.transposition_table.get_at_index(key, tt_index) {
            let entry_value = entry.value();
            match entry.flag() {
                TTFlag::Exact => return entry_value,
                TTFlag::LowerBound if entry_value >= beta => return entry_value,
                TTFlag::UpperBound if entry_value <= alpha => return entry_value,
                _ => {}
            }
            tt_best_move = entry.best_move();
        }

        let mut best_move_col = tt_best_move;
        let mut possible_moves_mask = possible_moves;
        let tt_move_available = tt_best_move < 7
            && (possible_moves_mask & Position::column_mask(tt_best_move as usize)) != 0;

        if tt_move_available {
            let mut new_position = position;
            new_position.play(tt_best_move as usize);
            let score = -self.negamax_wrapper(
                new_position,
                -beta,
                -alpha,
                opponent_winning_spots,
            );
            if score > alpha {
                alpha = score;
                best_move_col = tt_best_move;
            }
            if alpha >= beta {
                self.transposition_table
                    .put_at_index(key, alpha, TTFlag::LowerBound, best_move_col, depth, tt_index);
                return alpha;
            }

            // Mask out the move so we don't generate it again in the loop
            possible_moves_mask &= !Position::column_mask(tt_best_move as usize);
        }

        // Scores and sorts possible moves to explore the best ones first
        let moves = Self::generate_moves(position, possible_moves_mask);

        // Computes the scores of all possible next moves, keeping the best
        for column in moves {
            let mut new_position = position;
            new_position.play(column);
            let score = -self.negamax_wrapper(
                new_position,
                -beta,
                -alpha,
                opponent_winning_spots,
            );
            if score > alpha {
                alpha = score;
                best_move_col = column as u8;
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
        self.transposition_table
            .put_at_index(key, alpha, flag, best_move_col, depth, tt_index);

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