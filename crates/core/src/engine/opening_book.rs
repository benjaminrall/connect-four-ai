use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use serde::{Deserialize, Serialize};
use rayon::prelude::*;
use crate::{Position, Solver};

/// A cache that stores pre-computed scores for opening game positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningBook {
    pub map: HashMap<u64, i8>,
}

impl OpeningBook {
    /// Creates a new, empty opening book.
    pub fn new() -> OpeningBook {
        OpeningBook { map: HashMap::new() }
    }

    /// Creates an `OpeningBook` by deserialising from a byte slice.
    pub fn from_static_bytes(bytes: &'static [u8]) -> Result<OpeningBook, bincode::Error> {
        bincode::deserialize(bytes)
    }

    /// Looks up a position's score in the opening book.
    #[inline(always)]
    pub fn get(&self, position: &Position) -> Option<i8> {
        let key = position.get_key();
        // Check the primary key first, as it's the most likely hit.
        if let Some(&score) = self.map.get(&key) {
            return Some(score);
        }

        // If not found, check the mirrored key to handle symmetric positions.
        let mirrored_key = position.get_mirrored_key();
        if key != mirrored_key {
            if let Some(&score) = self.map.get(&mirrored_key) {
                return Some(score);
            }
        }

        None
    }

    /// Generates book entries for all positions up to a given depth.
    pub fn generate(&mut self, max_depth: usize) {
        let mut solver = Solver::new();
        solver.opening_book = Some(self.clone());
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        let start_pos = Position::new();
        queue.push_back((start_pos, 0)); // (position, depth)
        seen.insert(start_pos.get_key());


        let mut evaluated = 0;
        while let Some((pos, depth)) = queue.pop_front() {
            println!("pos: {:?}, depth: {}, evaluated: {}, queue_size: {}", pos, depth, evaluated, queue.len());

            // Solve and store the current position's score.
            let key = pos.get_key();
            if !self.map.contains_key(&key) {
                // solver.reset();
                let score = solver.solve(&pos);
                evaluated += 1;
                self.map.insert(key, score);
            }

            // Generate and enqueue all unique child positions.
            if depth < max_depth {
                let possible_moves = pos.possible();
                for col in 0..Position::WIDTH {
                    if (possible_moves & Position::column_mask(col)) > 0 {
                        let mut next_pos = pos;
                        next_pos.play(col);
                        // Use the canonical key (the smaller of the key and its mirror)
                        // to avoid storing symmetric positions twice.
                        let key = next_pos.get_key();
                        let mirrored_key = next_pos.get_mirrored_key();
                        let canonical_key = key.min(mirrored_key);

                        if seen.insert(canonical_key) {
                            queue.push_back((next_pos, depth + 1));
                        }
                    }
                }
            }
        }
    }

    /// Saves the opening book to a file using a compact binary format.
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self.map)?;
        Ok(())
    }

    /// Loads an opening book from a file.
    pub fn load(path: &Path) -> Result<OpeningBook, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let map = bincode::deserialize_from(reader)?;
        Ok(OpeningBook { map })
    }
}