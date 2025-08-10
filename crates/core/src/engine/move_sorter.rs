//! A simple container for potential Connect Four moves, sorted by a score heuristic.

use crate::Position;

/// Represents a single potential move entry.
#[derive(Debug, Copy, Clone, Default)]
pub struct MoveEntry {
    pub column: usize,
    pub score: u8,
}

/// A fixed-size container that stores a list of moves sorted by score.
pub struct MoveSorter {
    size: usize,
    entries: [MoveEntry; Position::WIDTH],
}

impl MoveSorter {
    /// Creates a new, empty `MoveSorter`.
    pub fn new() -> MoveSorter {
        Self::default()
    }

    /// Adds a move to the sorter and inserts it at the correct position.
    #[inline(always)]
    pub fn add(&mut self, column: usize, score: u8) {
        let mut pos = self.size;
        while pos > 0 && self.entries[pos - 1].score > score {
            self.entries[pos] = self.entries[pos - 1];
            pos -= 1;
        }
        self.entries[pos].column = column;
        self.entries[pos].score = score;
        self.size += 1;
    }
}

/// Implements the `Iterator` trait to allow looping over moves from best to worst.
impl Iterator for MoveSorter {
    type Item = usize;

    #[inline(always)]
    fn next(&mut self) -> Option<usize> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            Some(self.entries[self.size].column)
        }
    }
}

/// Default constructor for the `MoveSorter` struct.
impl Default for MoveSorter {
    fn default() -> MoveSorter {
        MoveSorter {
            size: 0,
            entries: [MoveEntry::default(); Position::WIDTH],
        }
    }
}