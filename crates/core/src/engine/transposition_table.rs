//! A transposition table implementation for storing and retrieving game state evaluations.

/// A flag indicating the nature of the stored score relative to the true score.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TTFlag {
    /// The entry is empty or invalid.
    #[default]
    None = 0,
    /// The value is a lower bound (score >= value).
    LowerBound = 1,
    /// The value is an upper bound (score <= value).
    UpperBound = 2,
    /// The value is exact (score == value).
    Exact = 3,
}

/// A bit-packed 8-byte transposition table entry.
///
/// Layout (64 bits):
/// [0..49]   Key (49 bits)
/// [49..55]  Score (6 bits)  (value + 32)
/// [55..57]  Flag (2 bits)
/// [57..60]  Move (3 bits)
/// [60..64]  Depth (4 bits)
#[derive(Debug, Default, Copy, Clone)]
#[repr(transparent)]
pub struct TTEntry {
    data: u64,
}

impl TTEntry {
    const KEY_MASK: u64 = (1 << 49) - 1;

    #[inline(always)]
    fn new(key: u64, value: i8, flag: TTFlag, best_move: u8, depth: u8) -> Self {
        // 1. Key (49 bits)
        let k_part = key & Self::KEY_MASK;
        // 2. Score (6 bits). Map -32..31 to 0..63.
        let s_part = ((value + 32) as u64 & 0x3F) << 49;
        // 3. Flag (2 bits).
        let f_part = (flag as u64) << 55;
        // 4. Move (3 bits).
        let m_part = ((best_move as u64) & 0x7) << 57;
        // 5. Depth (4 bits).
        // Store scaled depth to fit 0..42 into 4 bits.
        let d_part = (((depth as u64) / 3).min(15)) << 60;

        TTEntry {
            data: k_part | s_part | f_part | m_part | d_part,
        }
    }

    #[inline(always)]
    pub fn key(&self) -> u64 {
        self.data & Self::KEY_MASK
    }

    #[inline(always)]
    pub fn value(&self) -> i8 {
        ((self.data >> 49) & 0x3F) as i8 - 32
    }

    #[inline(always)]
    pub fn flag(&self) -> TTFlag {
        match (self.data >> 55) & 0x3 {
            1 => TTFlag::LowerBound,
            2 => TTFlag::UpperBound,
            3 => TTFlag::Exact,
            _ => TTFlag::None,
        }
    }

    #[inline(always)]
    pub fn best_move(&self) -> u8 {
        ((self.data >> 57) & 0x7) as u8
    }

    #[inline(always)]
    pub fn depth_code(&self) -> u8 {
        ((self.data >> 60) & 0xF) as u8
    }
}

/// A bucket containing two entries to handle hash collisions.
///
/// This implements a 2-way set-associative cache.
#[derive(Debug, Copy, Clone, Default)]
struct TTBucket {
    entries: [TTEntry; 2],
}

/// A transposition table that stores results from previous searches to avoid
/// re-computing evaluations for the same game state.
#[derive(Debug)]
pub struct TranspositionTable {
    /// A list of buckets.
    entries: Vec<TTBucket>,
}

impl TranspositionTable {
    pub const INDEX_BITS: usize = 19;
    pub const MAX_SIZE: usize = 1 << Self::INDEX_BITS;
    const INDEX_MASK: u64 = (Self::MAX_SIZE as u64) - 1;

    const KEY_BITS: usize = 49;
    const R1: u32 = Self::INDEX_BITS as u32;
    const R2: u32 = (Self::KEY_BITS - Self::INDEX_BITS) as u32;

    /// Creates a new empty transposition table, allocating space for all entries.
    pub fn new() -> TranspositionTable {
        Self::default()
    }

    #[inline(always)]
    pub fn index(&self, key: u64) -> usize {
        let mixed = key ^ key.rotate_right(Self::R1) ^ key.rotate_right(Self::R2);
        (mixed & Self::INDEX_MASK) as usize
    }

    /// Clears the table.
    #[inline(always)]
    pub fn reset(&mut self) {
        self.entries.fill(TTBucket::default());
    }

    /// Retrieves an entry from the table if it exists and is valid.
    #[inline(always)]
    pub fn get(&self, key: u64) -> Option<TTEntry> {
        let index = self.index(key);
        self.get_at_index(key, index)
    }

    /// Retrieves an entry from the table if it exists and is valid, using a precomputed index.
    #[inline(always)]
    pub fn get_at_index(&self, key: u64, index: usize) -> Option<TTEntry> {
        debug_assert!(index < Self::MAX_SIZE);
        let bucket = unsafe { self.entries.get_unchecked(index) };

        let entry0 = bucket.entries[0];
        if entry0.key() == key {
            return Some(entry0);
        }

        let entry1 = bucket.entries[1];
        if entry1.key() == key {
            return Some(entry1);
        }

        None
    }
    
    /// Stores a new entry in the table using a tiered replacement strategy.
    #[inline(always)]
    pub fn put(&mut self, key: u64, value: i8, flag: TTFlag, best_move: u8, depth: u8) {
        let index = self.index(key);
        self.put_at_index(key, value, flag, best_move, depth, index);
    }

    /// Stores a new entry in the table using a precomputed index and a tiered replacement strategy.
    #[inline(always)]
    pub fn put_at_index(
        &mut self,
        key: u64,
        value: i8,
        flag: TTFlag,
        best_move: u8,
        depth: u8,
        index: usize,
    ) {
        debug_assert!(index < Self::MAX_SIZE);
        let bucket = unsafe { self.entries.get_unchecked_mut(index) };
        let new_entry = TTEntry::new(key, value, flag, best_move, depth);

        let slot0 = bucket.entries[0];
        if slot0.key() == key {
            bucket.entries[0] = new_entry;
            return;
        }

        let slot1 = bucket.entries[1];
        if slot1.key() == key {
            bucket.entries[1] = new_entry;
            return;
        }

        if new_entry.depth_code() >= slot0.depth_code() {
            bucket.entries[1] = slot0;
            bucket.entries[0] = new_entry;
        } else {
            bucket.entries[1] = new_entry;
        }
    }

    /// Prefetches the bucket for the given key into the L1 cache.
    #[inline(always)]
    pub fn prefetch(&self, key: u64) {
        #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
        {
            let index = self.index(key);
            self.prefetch_index(index);
        }
    }


    #[inline(always)]
    pub fn prefetch_index(&self, index: usize) {
        debug_assert!(index < Self::MAX_SIZE);

        #[cfg(target_arch = "x86_64")]
        unsafe {
            let base_ptr = self.entries.as_ptr();
            let bucket_ptr = base_ptr.add(index) as *const i8;
            // _MM_HINT_T0: Prefetch into all cache levels (L1/L2/L3)
            use core::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
            _mm_prefetch(bucket_ptr, _MM_HINT_T0);
        }

        #[cfg(target_arch = "aarch64")]
        unsafe {
            let base_ptr = self.entries.as_ptr();
            let bucket_ptr = base_ptr.add(index) as *const i8;
            // PLDL1KEEP: Prefetch for Load, into L1 cache, Keep in cache (temporal).
            use core::arch::aarch64::{_prefetch, _PREFETCH_READ, _PREFETCH_LOCALITY0};
            _prefetch(bucket_ptr, _PREFETCH_READ, _PREFETCH_LOCALITY0);
        }
    }
}

/// Default constructor for the `TranspositionTable` struct.
impl Default for TranspositionTable {
    fn default() -> TranspositionTable {
        TranspositionTable {
            entries: vec![TTBucket::default(); Self::MAX_SIZE],
        }
    }
}
