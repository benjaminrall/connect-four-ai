
type Key = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum TTFlag {
    #[default]
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TTEntry {
    pub key: Key,
    pub value: i8,
    pub flag: TTFlag,
    pub depth: u8,
    pub is_valid: bool,
}

#[derive(Debug)]
pub struct TranspositionTable {
    entries: Vec<TTEntry>
}

impl TranspositionTable {
    pub const MAX_SIZE: usize = (1 << 23) + 9;

    pub fn new() -> TranspositionTable {
        TranspositionTable {
            entries: vec![TTEntry::default(); Self::MAX_SIZE],
        }
    }

    pub fn index(&self, key: u64) -> usize {
        key as usize % Self::MAX_SIZE
    }

    pub fn reset(&mut self) {
        self.entries.iter_mut().for_each(|x| x.is_valid = false);
    }

    pub fn put(&mut self, key: u64, value: i8, flag: TTFlag, depth: u8) {
        let pos = self.index(key);
        self.entries[pos].key = key as Key;
        self.entries[pos].value = value;
        self.entries[pos].flag = flag;
        self.entries[pos].depth = depth;
        self.entries[pos].is_valid = true;
    }

    pub fn get(&self, key: u64) -> Option<&TTEntry> {
        let pos = self.index(key);
        let entry = &self.entries[pos];
        if entry.key == key as Key && entry.is_valid {
            Some(entry)
        } else {
            None
        }
    }
}