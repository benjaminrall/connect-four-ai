//! A simple container for potential Connect Four moves, sorted by a score heuristic.

use std::mem::MaybeUninit;

use crate::Position;

/// Represents a single potential move entry.
#[derive(Debug, Copy, Clone, Default)]
pub struct MoveEntry {
    pub column: u8,
    pub score: u8,
}

/// A fixed-size container that stores a list of moves sorted by score.
pub struct MoveSorter {
    size: usize,
    entries: [MaybeUninit<MoveEntry>; Position::WIDTH],
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
        unsafe {
            while pos > 0 && self.entries[pos - 1].assume_init_ref().score > score {
                let prev = *self.entries[pos - 1].assume_init_ref();
                self.entries[pos].write(prev);
                pos -= 1;
            }
            self.entries[pos].write(MoveEntry {
                column: column as u8,
                score,
            });
        }
        self.size += 1;
    }

    /// Returns the next move in the sorter without removing it.
    /// Used for prefetching the next position's TT entry.
    #[inline(always)]
    pub fn peek(&self) -> Option<u8> {
        if self.size == 0 {
            None
        } else {
            unsafe { Some(self.entries[self.size - 1].assume_init_ref().column) }
        }
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
            unsafe { Some(self.entries[self.size].assume_init_ref().column as usize) }
        }
    }
}

/// Default constructor for the `MoveSorter` struct.
impl Default for MoveSorter {
    fn default() -> MoveSorter {
        MoveSorter {
            size: 0,
            entries: [MaybeUninit::uninit(); Position::WIDTH],
        }
    }
}