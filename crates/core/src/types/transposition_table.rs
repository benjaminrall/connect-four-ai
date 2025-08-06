
type Key = u32;

#[derive(Debug)]
pub struct TranspositionTable {
    keys: Vec<Key>,
    values: Vec<u8>,
}

impl TranspositionTable {
    pub const MAX_SIZE: usize = (1 << 23) + 9;

    pub fn new() -> TranspositionTable {
        TranspositionTable {
            keys: vec![0; Self::MAX_SIZE],
            values: vec![0; Self::MAX_SIZE],
        }
    }

    pub fn index(&self, key: u64) -> usize {
        key as usize % Self::MAX_SIZE
    }

    pub fn reset(&mut self) {
        self.keys.iter_mut().for_each(|x| *x = 0);
    }

    pub fn put(&mut self, key: u64, value: u8) {
        let pos = self.index(key);
        self.keys[pos] = key as Key;
        self.values[pos] = value;
    }

    pub fn get(&self, key: u64) -> Option<u8> {
        let pos = self.index(key);
        if self.keys[pos] == key as Key {
            Some(self.values[pos])
        } else {
            None
        }
    }
}