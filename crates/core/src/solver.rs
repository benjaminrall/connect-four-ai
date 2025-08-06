use std::num::NonZeroUsize;
use lru::LruCache;
use crate::{MoveSorter, Position, TranspositionTable};

#[derive(Debug)]
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

            let r = self.negamax(position, med, med + 1);

            // println!("{} {} {} {}", min, med, max, r);

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

    pub fn negamax(&mut self, position: &Position, mut alpha: i8, mut beta: i8) -> i8 {
        self.explored_positions += 1;

        // If there are no possible non-losing moves, then the opponent is guaranteed to win
        let next = position.possible_non_losing_moves();
        if next == 0 {
            return -((Position::BOARD_SIZE - position.get_moves()) as i8) / 2;
        }

        // Checks for a drawn game
        if position.get_moves() == Position::BOARD_SIZE {
            return 0;
        }

        // Lower bound of our score as opponent cannot win next move
        let min = -((Position::BOARD_SIZE - position.get_moves()) as i8 - 2) / 2;
        if alpha < min {
            if min >= beta { return min}
            alpha = min;
        }

        // Upper bound of our score as we cannot win immediately
        let mut max = ((Position::BOARD_SIZE - position.get_moves()) as i8 - 1) / 2;
        if beta > max {
            if alpha >= max { return max }
            beta = max;
        }

        // Transposition table fetches
        let key = position.get_key();
        if let Some(value) = self.transposition_table.get(key).map(|v| v as i8) {
            if value > Position::MAX_SCORE - Position::MIN_SCORE - 2 {
                let min = value + 2 * Position::MIN_SCORE - Position::MAX_SCORE - 2;
                if alpha < min {
                    if min >= beta { return min }
                    alpha = min;
                }
            } else {
                let max = value + Position::MIN_SCORE - 1;
                if beta > max {
                    if alpha >= max { return max }
                    beta = max;
                }
            }
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
            let score = -self.negamax(&new_position, -beta, -alpha);

            if score >= beta {
                self.transposition_table.put(key, (score + Position::MAX_SCORE - 2 * Position::MIN_SCORE + 2) as u8);
                return score;
            }

            if score > alpha {
                alpha = score;
            }
        }

        for i in 0..Position::WIDTH {
            let column = Self::COLUMNS[i];
            if next & Position::column_mask(column) > 0 {

            }
        }

        // Saves the upper bound of the position
        self.transposition_table.put(key, (alpha - Position::MIN_SCORE + 1) as u8);
        alpha
    }
}

impl Default for Solver {
    fn default() -> Solver {
        Solver {
            explored_positions: 0,
            transposition_table: TranspositionTable::new(),
        }
    }
}



