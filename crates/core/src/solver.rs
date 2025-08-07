use std::cmp::PartialEq;
use std::num::NonZeroUsize;
use lru::LruCache;
use crate::{MoveSorter, Position, TTFlag, TranspositionTable};

#[derive(Debug, Default)]
pub struct Solver {
    pub explored_positions: usize,
    transposition_table: TranspositionTable,
}


impl Solver {
    const COLUMNS: [usize; Position::WIDTH] = const {
        let mut columns = [0; Position::WIDTH];
        let mut i = 0;
        while i < Position::WIDTH {
            columns[i] = (Position::WIDTH as i32 / 2 + (1 - 2 * (i as i32 % 2)) * (i as i32 + 1) / 2) as usize;
            i += 1;
        }
        columns
    };

    pub fn reset(&mut self) {
        self.explored_positions = 0;
        self.transposition_table.reset();
    }

    pub fn solve(&mut self, position: &Position) -> i8 {
        let mut min = -((Position::BOARD_SIZE - position.get_moves()) as i8) / 2;
        let mut max = (Position::BOARD_SIZE + 1 - position.get_moves()) as i8 / 2;

        while min < max {
            let mut med = min + (max - min) / 2;

            if med <= 0 && min / 2 < med {
                med = min / 2
            } else if med >= 0 && max / 2 > med {
                med = max / 2
            }

            let r = self.negamax(position, (Position::BOARD_SIZE - position.get_moves()) as u8, med, med + 1);

            if r <= med {
                max = r
            } else {
                min = r
            }
        }

        min
    }

    fn top_level_search(&mut self, position: &Position, alpha: i8, beta: i8) -> (i8, usize) {
        (0, 0)
    }

    pub fn negamax(&mut self, position: &Position, depth: u8, mut alpha: i8, mut beta: i8) -> i8 {
        self.explored_positions += 1;

        // Checks for a drawn game
        if depth == 0 {
            return 0;
        }

        // Transposition table shenanigans
        let original_alpha = alpha;
        let key = position.get_key();
        if let Some(entry) = self.transposition_table.get(key) {
            if entry.depth >= depth {
                if entry.flag == TTFlag::Exact {
                    return entry.value
                } else if entry.flag == TTFlag::LowerBound && entry.value >= beta {
                    return entry.value
                } else if entry.flag == TTFlag::UpperBound && entry.value <= alpha {
                    return entry.value
                }
            }
        }

        // If there are no possible non-losing moves, then the opponent is guaranteed to win
        let next = position.possible_non_losing_moves();
        if next == 0 {
            return -((Position::BOARD_SIZE - position.get_moves()) as i8) / 2;
        }

        // Lower bound of the as opponent cannot win next move
        let min = -((Position::BOARD_SIZE - position.get_moves()) as i8 - 2) / 2;
        if alpha < min {
            if min >= beta { return min}
            alpha = min;
        }

        // Upper bound of the score as we cannot win immediately
        let mut max = ((Position::BOARD_SIZE - position.get_moves()) as i8 - 1) / 2;
        if beta > max {
            if alpha >= max { return max }
            beta = max;
        }

        // Scores and sorts possible moves
        let mut moves = MoveSorter::new();
        for i in (0..Position::WIDTH).rev() {
            let column = Self::COLUMNS[i];
            let move_bit = next & Position::column_mask(column);
            if move_bit > 0 {
                moves.add(column, position.score_move(move_bit))
            }
        }

        // Computes the scores of all possible next moves, keeping the best
        while let Some(column) = moves.next() {
            let mut new_position = *position;
            new_position.play(column);

            let score = -self.negamax(&new_position, depth - 1, -beta, -alpha);

            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                break;
            }
        }

        // Saves the position
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
