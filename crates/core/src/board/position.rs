//! A compact, computationally efficient bitboard representation of Connect 4 positions.

use crate::PositionParsingError;

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
/// The extra row of bits at the top identifies full columns and prevents bits from overflowing
/// into the next column. For computational efficiency, positions are stored in practice using two
/// `u64` numbers: one to store a mask of all occupied tiles, and the other to store a mask of the
/// current player's tiles.
#[derive(Debug, Copy, Clone)]
pub struct Position {
    /// A mask of the current player's tiles.
    position: u64,
    /// A mask of all occupied tiles.
    mask: u64,
    /// The number of moves taken to reach the position.
    moves: usize,
}

impl Position {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;
    pub const BOARD_SIZE: usize = Self::WIDTH * Self::HEIGHT;
    pub const MIN_SCORE: i8 = -(Self::BOARD_SIZE as i8) / 2 + 3;
    pub const MAX_SCORE: i8 = (Self::BOARD_SIZE as i8 + 1) / 2 - 3;

    /// A mask for the bottom row of the board.
    const BOTTOM_MASK: u64 = const {
        let mut mask = 0;
        let mut i = 0;
        while i < Self::WIDTH {
            mask |= Position::bottom_mask_col(i);
            i += 1;
        }
        mask
    };

    /// A mask for all positions within the board, excluding the extra overflow row.
    const BOARD_MASK: u64 = Self::BOTTOM_MASK * ((1 << Self::HEIGHT) - 1);

    /// Creates a new `Position` instance for the initial position of the game.
    pub fn new() -> Position {
        Position {
            position: 0,
            mask: 0,
            moves: 0,
        }
    }

    /// Parses a `Position` from a string representation of the Connect Four board.
    ///
    /// The input string should contain exactly 42 characters from the set `['.', 'o', 'x']`,
    /// representing the board row by row from the top-left to the bottom-right. All other
    /// characters are ignored. 'x' is treated as the current player, and 'o' as the opponent.
    /// This method assumes that a correctly formatted board string is a valid game position.
    /// Invalid game positions will lead to undefined behaviour.
    ///
    /// # Arguments
    ///
    /// * `board_string`: A string slice representing the board state.
    ///
    /// # Returns
    ///
    /// On success, returns a `Result` containing the parsed `Position`.
    ///
    /// # Errors
    ///
    /// Returns a `PositionParsingError` if the input string is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    ///  use connect_four_ai::Position;
    ///
    ///  // A typical board state, represented as a string
    ///  let board_string = "\
    ///     .......
    ///     ...o...
    ///     ..xx...
    ///     ..ox...
    ///     ..oox..
    ///     ..oxxo.
    ///  ";
    ///
    ///  // Parses the string as a `Position` instance
    ///  let pos = Position::from_board_string(board_string).unwrap();
    ///  assert_eq!(pos.get_moves(), 12)
    /// ```
    pub fn from_board_string(board_string: &str) -> Result<Position, PositionParsingError> {
        let chars: Vec<char> = board_string
            .to_lowercase()
            .chars()
            .filter(|c| matches!(c, '.' | 'o' | 'x'))
            .collect();

        // Validates that there is the exact number of characters required for a full board
        if chars.len() != Self::BOARD_SIZE {
            return Err(PositionParsingError::InvalidBoardStringLength {
                actual: chars.len(),
                expected: Self::BOARD_SIZE,
            });
        }

        // Values required to construct a `Position`
        let mut position = 0;
        let mut mask = 0;
        let mut moves = 0;

        // Loops through the board string's characters to construct the `Position` bitboards
        for (i, &current_char) in chars.iter().enumerate() {
            if current_char == '.' {
                continue;
            }

            // Calculates board coordinates from the current index
            let row = Self::HEIGHT - (i / Self::WIDTH) - 1;
            let col = i % Self::WIDTH;

            // Calculates the current character's corresponding bit index in the bitboards
            let bit_index = row + col * (Self::HEIGHT + 1);

            // Sets a '1' in the relevant bit if the condition is true, otherwise '0'
            let position_bit = (current_char == 'x') as u64;

            // Uses a bitwise OR to set the calculated bits in the appropriate bitboards
            position |= position_bit << bit_index;
            mask |= 1 << bit_index;
            moves += 1;
        }

        Ok(Position { position, mask, moves })
    }


    /// Parses a `Position` from a string of 1-indexed moves.
    ///
    /// The input string should contain a sequence of columns played, indexed from 1.
    ///
    /// # Arguments
    ///
    /// * `moves`: A string slice containing the move sequence.
    ///
    /// # Returns
    ///
    /// On success, returns a `Result` containing the parsed `Position`.
    ///
    /// # Errors
    ///
    /// Returns a `PositionParsingError` if the move sequence is invalid.
    ///
    /// # Example
    ///
    /// ```rust
    ///  use connect_four_ai::Position;
    ///
    ///  // A typical board state, represented as a sequence of moves
    ///  let moves = "444343533654";
    ///
    ///  // Parses the sequence as a `Position` instance
    ///  let pos = Position::from_moves(moves).unwrap();
    ///  assert_eq!(pos.get_moves(), 12)
    /// ```
    pub fn from_moves(move_sequence: &str) -> Result<Position, PositionParsingError> {
        let mut pos = Position::new();

        // Applies the move sequence to the position in order
        for (i, c) in move_sequence.chars().enumerate() {
            match c.to_digit(10)
                .map(|digit| (digit - 1) as usize) {
                Some(col @ 0..Self::WIDTH) => {
                    // Validates the move
                    if !pos.is_playable(col) {
                        return Err(PositionParsingError::InvalidFullColumnMove { column: col + 1, index: i })
                    }
                    if pos.is_winning_move(col) {
                        return Err(PositionParsingError::InvalidWinningMove { column: col + 1, index: i  })
                    }

                    pos.play(col);
                },
                Some(col) => return Err(PositionParsingError::InvalidColumn { column: col + 1, index: i  }),
                None => return Err(PositionParsingError::InvalidCharacter { character: c, index: i  }),
            }
        }

        Ok(pos)
    }


    /// Returns the number of moves played to reach the current position.
    #[inline(always)]
    pub fn get_moves(&self) -> usize {
        self.moves
    }


    /// Returns a unique key for the current position.
    #[inline(always)]
    pub fn get_key(&self) -> u64 {
        self.position + self.mask
    }


    /// Returns the mirrored key for the current position.
    pub fn get_mirrored_key(&self) -> u64 {
        let key = self.get_key();
        let mut mirrored_key = 0;

        for col in 0..Self::WIDTH {
            let mirrored_col = Self::WIDTH - 1 - col;
            let column_data = key & Self::column_mask(col);
            let shift = (mirrored_col as isize - col as isize).abs() * (Self::HEIGHT + 1) as isize;

            mirrored_key |= if col < mirrored_col {
                column_data << shift
            } else {
                column_data >> shift
            };
        }

        mirrored_key
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
    #[inline(always)]
    pub fn is_playable(&self, col: usize) -> bool {
        self.mask & Self::top_mask_col(col) == 0
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
    pub fn is_winning_move(&self, col: usize) -> bool {
        self.winning_positions() & self.possible() & Self::column_mask(col) > 0
    }


    /// Plays a move in the given column.
    ///
    /// # Arguments
    ///
    /// * `col`: 0-based index of a playable column.
    #[inline(always)]
    pub fn play(&mut self, col: usize) {
        // Switches the bits of the current and opponent player
        self.position ^= self.mask;

        // Adds an extra mask bit to the played column
        self.mask |= self.mask + Self::bottom_mask_col(col);

        self.moves += 1;
    }


    /// Returns a mask for the possible moves the current player can make.
    #[inline(always)]
    pub fn possible(&self) -> u64 {
        (self.mask + Self::BOTTOM_MASK) & Self::BOARD_MASK
    }


    /// Returns a mask for the possible non-losing moves the current player can make.
    pub fn possible_non_losing_moves(&self) -> u64 {
        let mut possible = self.possible();
        let opponent_wins = self.opponent_winning_positions();

        // Checks if there are any forced moves to avoid the opponent winning
        let forced_moves = possible & opponent_wins;
        if forced_moves > 0 {
            if forced_moves & (forced_moves - 1) > 0 {
                // If the opponent has two winning moves then they cannot be stopped
                return 0
            } else {
                possible = forced_moves;
            }
        }

        // Avoid playing below any of the opponent's winning positions
        possible & !(opponent_wins >> 1)
    }


    /// Returns a mask for the current player's winning positions.
    fn winning_positions(&self) -> u64 {
        Self::compute_winning_positions(self.position, self.mask)
    }


    /// Returns a mask for the opponent's winning positions.
    fn opponent_winning_positions(&self) -> u64 {
        Self::compute_winning_positions(self.position ^ self.mask, self.mask)
    }


    /// Computes a mask for all of a player's winning positions.
    ///
    /// Equivalent to a mask of all open-ended 3-alignments,
    /// including unreachable floating positions.
    ///
    /// # Arguments
    ///
    /// * `position`: Bitmask for a player's occupied positions.
    /// * `mask`: Bitmask for all occupied positions.
    ///
    /// # Returns
    ///
    /// A bitmask with ones in all positions that a piece could be played by the player to win the game.
    fn compute_winning_positions(position: u64, mask: u64) -> u64 {
        // Vertical alignment
        let mut r = (position << 1) & (position << 2) & (position << 3);

        // Horizontal alignment
        let mut p = (position << (Self::HEIGHT + 1)) & (position << 2 * (Self::HEIGHT + 1));
        r |= p & (position << 3 * (Self::HEIGHT + 1));
        r |= p & (position >> (Self::HEIGHT + 1));
        p >>= 3 * (Self::HEIGHT + 1);
        r |= p & (position << (Self::HEIGHT + 1));
        r |= p & (position >> 3 * (Self::HEIGHT + 1));

        // Diagonal alignment 1
        let mut p = (position << Self::HEIGHT) & (position << 2 * Self::HEIGHT);
        r |= p & (position << 3 * Self::HEIGHT);
        r |= p & (position >> Self::HEIGHT);
        p >>= 3 * Self::HEIGHT;
        r |= p & (position << Self::HEIGHT);
        r |= p & (position >> 3 * Self::HEIGHT);

        // Diagonal alignment 2
        let mut p = (position << (Self::HEIGHT + 2)) & (position << 2 * (Self::HEIGHT + 2));
        r |= p & (position << 3 * (Self::HEIGHT + 2));
        r |= p & (position >> (Self::HEIGHT + 2));
        p >>= 3 * (Self::HEIGHT + 2);
        r |= p & (position << (Self::HEIGHT + 2));
        r |= p & (position >> 3 * (Self::HEIGHT + 2));

        r & (Self::BOARD_MASK ^ mask)
    }


    /// Scores a possible move by counting the number of winning spots
    /// the player has after playing it.
    ///
    /// # Arguments
    ///
    /// * `move_bit`: A possible move, given as a bitmask with a single one in the position of the
    /// new piece.
    ///
    /// # Returns
    ///
    /// The move's score.
    pub fn score_move(&self, move_bit: u64) -> u8 {
        Self::compute_winning_positions(self.position | move_bit, self.mask).count_ones() as u8
    }

    /// Returns a mask for the top element of the given column.
    ///
    /// # Arguments
    ///
    /// * `col`: 0-based index of a column.
    ///
    /// # Returns
    ///
    /// A bitmask with a singular one in the top of cell the column.
    #[inline(always)]
    pub const fn top_mask_col(col: usize) -> u64 {
        1 << (Self::HEIGHT - 1 + col * (Self::HEIGHT + 1))
    }

    /// Returns a mask for the bottom element of the given column.
    ///
    /// # Arguments
    ///
    /// * `col`: 0-based index of a column.
    ///
    /// # Returns
    ///
    /// A bitmask with a single one in the bottom cell of the column.
    #[inline(always)]
    pub const fn bottom_mask_col(col: usize) -> u64 {
        1 << (col * (Self::HEIGHT + 1))
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
    #[inline(always)]
    pub const fn column_mask(col: usize) -> u64 {
        ((1 << Self::HEIGHT) - 1) << (col * (Self::HEIGHT + 1))
    }
}