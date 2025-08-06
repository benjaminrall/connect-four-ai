use crate::Position;

#[derive(Debug, Copy, Clone, Default)]
pub struct MoveEntry {
    pub column: usize,
    pub score: u8,
}

pub struct MoveSorter {
    size: usize,
    entries: [MoveEntry; Position::WIDTH],
}

impl MoveSorter {
    pub fn new() -> MoveSorter {
        MoveSorter {
            size: 0,
            entries: [MoveEntry::default(); Position::WIDTH],
        }
    }

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

    pub fn next(&mut self) -> Option<usize> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            Some(self.entries[self.size].column)
        }
    }
}