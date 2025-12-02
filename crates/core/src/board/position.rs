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
    pub position: u64,
    /// A mask of all occupied tiles.
    pub mask: u64,
}

impl Position {
    pub const WIDTH: usize = 7;
    pub const HEIGHT: usize = 6;
    pub const BOARD_SIZE: usize = Self::WIDTH * Self::HEIGHT;
    pub const CENTRE: usize = Self::WIDTH / 2;
    pub const MIN_SCORE: i8 = -(Self::BOARD_SIZE as i8) / 2 + 3;
    pub const MAX_SCORE: i8 = (Self::BOARD_SIZE as i8 + 1) / 2 - 3;

    /// A mask for the bottom row of the board.
    const BOTTOM_MASK: u64 = const {
        let mut mask = 0;
        let mut i = 0;
        while i < Self::WIDTH {
            mask |= Self::bottom_mask_col(i);
            i += 1;
        }
        mask
    };

    /// A mask for all positions within the board, excluding the extra overflow row.
    const BOARD_MASK: u64 = Self::BOTTOM_MASK * ((1 << Self::HEIGHT) - 1);

    /// Creates a new `Position` instance for the initial state of the game.
    pub fn new() -> Position {
        Self::default()
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
        }

        Ok(Position { position, mask })
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
        let mut pos = Self::new();

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
        self.mask.count_ones() as usize
    }

    /// Returns the unique key for the current position.
    ///
    /// This key is unique to each pair of horizontally symmetrical positions, as these
    /// positions will always have the same solution.
    #[inline(always)]
    pub fn get_key(&self) -> u64 {
        let key = self.position + self.mask;
        let mirrored_key = Self::mirror_key(key);
        key.min(mirrored_key)
    }

    #[inline(always)]
    fn mirror_key(mut key: u64) -> u64 {
        let left_block = key & 0x1FFFFF;
        let right_block = (key >> 28) & 0x1FFFFF;
        let center = key & 0x0FE00000;
        key = center | (left_block << 28) | right_block;
        let mask_pairs = 0x7F000007F;
        let lower = key & mask_pairs;
        let upper = key & (mask_pairs << 14);
        key &= !(mask_pairs | (mask_pairs << 14));
        key | (lower << 14) | (upper >> 14)
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

    /// Indicates whether the current player can win with their next move.
    pub fn can_win_next(&self) -> bool {
        self.winning_positions() & self.possible() > 0
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
    pub fn winning_positions(&self) -> u64 {
        Self::compute_winning_positions(self.position, self.mask)
    }

    /// Returns a mask for the opponent's winning positions.
    pub fn opponent_winning_positions(&self) -> u64 {
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
        // Vertical alignment (d=1)
        // Only one pattern possible (XXX.) because of gravity.
        let mut r = (position << 1) & (position << 2) & (position << 3);

        // Horizontal alignment (d=7)
        let p = (position << 7) & (position << 14);
        r |= p & ((position << 21) | (position >> 7));
        let p2 = p >> 21;
        r |= p2 & ((position >> 21) | (position << 7));

        // Diagonal alignment 1 (d=6)
        let p = (position << 6) & (position << 12);
        r |= p & ((position << 18) | (position >> 6));
        let p2 = p >> 18;
        r |= p2 & ((position >> 18) | (position << 6));

        // Diagonal alignment 2 (d=8)
        let p = (position << 8) & (position << 16);
        r |= p & ((position << 24) | (position >> 8));
        let p2 = p >> 24;
        r |= p2 & ((position >> 24) | (position << 8));

        r & (Self::BOARD_MASK ^ mask)
    }

    /// Scores a possible move by counting the number of winning spots
    /// the player has after playing it.
    ///
    /// # Arguments
    ///
    /// * `move_bit`: A possible move, given as a bitmask with a single one in the position of the
    ///   new piece.
    ///
    /// # Returns
    ///
    /// The move's score.
    pub fn score_move(&self, move_bit: u64) -> u8 {
        Self::compute_winning_positions(self.position | move_bit, self.mask).count_ones() as u8
    }

    /// Indicates whether the current position has been won by either player.
    pub fn is_won_position(&self) -> bool {
        Self::compute_won_position(self.position) || Self::compute_won_position(self.position ^ self.mask)
    }

    /// Computes whether the given position contains a 4-alignment.
    fn compute_won_position(position: u64) -> bool {
        // Horizontal alignment
        let m = position & (position >> (Self::HEIGHT+1));
        if m & (m >> (2*(Self::HEIGHT+1))) > 0 { return true; }

        // Diagonal alignment 1
        let m = position & (position >> Self::HEIGHT);
        if m & (m >> (2*Self::HEIGHT)) > 0 { return true; }

        // Diagonal alignment 2
        let m = position & (position >> (Self::HEIGHT+2));
        if m & (m >> (2*(Self::HEIGHT+2))) > 0 { return true; }

        // Vertical alignment
        let m = position & (position >> 1);
        if m & (m >> 2) > 0 { return true; }

        false
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
    const fn top_mask_col(col: usize) -> u64 {
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
    const fn bottom_mask_col(col: usize) -> u64 {
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

/// Default constructor for the `Position` struct.
impl Default for Position {
    fn default() -> Position {
        Position {
            position: 0,
            mask: 0,
        }
    }
}