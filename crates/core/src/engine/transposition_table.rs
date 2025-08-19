//! A transposition table implementation for storing and retrieving game state evaluations.

/// A flag indicating what kind of information a transposition table entry represents.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TTFlag {
    /// Flag for when the value is the exact score of a position.
    #[default]
    Exact,
    /// Flag for when the value is a lower bound for the position's score.
    LowerBound,
    /// Flag for when the value is an upper bound for the position's score.
    UpperBound,
}

/// Represents a single entry in the transposition table.
#[derive(Debug, Default, Copy, Clone)]
pub struct TTEntry {
    /// The 32-bit key used to verify the entry.
    pub key: u32,
    /// The evaluated score of the position.
    pub value: i8,
    /// The type of score held in the entry.
    pub flag: TTFlag,
    /// The search depth at which this entry was recorded.
    pub depth: u8,
    /// The age of the transposition table when the entry was created.
    pub age: u8,
}

/// A transposition table that stores results from previous searches to avoid
/// re-computing evaluations for the same game state.
#[derive(Debug)]
pub struct TranspositionTable {
    /// A list of table entries
    entries: Vec<TTEntry>,
    /// The current age of the table, used to invalidate old entries.
    age: u8,
}

impl TranspositionTable {
    /// The number of entries in the table. A large prime number is chosen to help avoid collisions.
    pub const MAX_SIZE: usize = (1 << 23) + 9;

    /// Creates a new empty transposition table, allocating space for all entries.
    pub fn new() -> TranspositionTable {
        Self::default()
    }

    /// Calculates the table index for a given position's key.
    #[inline(always)]
    pub fn index(&self, key: u64) -> usize {
        (key % Self::MAX_SIZE as u64) as usize
    }

    /// Clears the table by incrementing the current age.
    pub fn reset(&mut self) {
        self.age = self.age.wrapping_add(1);
    }

    /// Stores a new entry in the table, overwriting any existing entry at the calculated index.
    pub fn put(&mut self, key: u64, value: i8, flag: TTFlag, depth: u8) {
        let pos = self.index(key);
        self.entries[pos].key = key as u32;
        self.entries[pos].value = value;
        self.entries[pos].flag = flag;
        self.entries[pos].depth = depth;
        self.entries[pos].age = self.age;
    }

    /// Retrieves an entry from the table if it exists and is valid.
    pub fn get(&self, key: u64) -> Option<&TTEntry> {
        let pos = self.index(key);
        let entry = &self.entries[pos];

        // Checks that both the key and age match to ensure correctness
        if entry.key == key as u32 && entry.age == self.age {
            Some(entry)
        } else {
            None
        }
    }
}

/// Default constructor for the `TranspositionTable` struct.
impl Default for TranspositionTable {
    fn default() -> TranspositionTable {
        TranspositionTable {
            entries: vec![TTEntry::default(); Self::MAX_SIZE as usize],
            age: 0,
        }
    }
}