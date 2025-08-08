//! An opening book for Connect Four, which stores pre-computed scores for opening game positions.

use crate::Position;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// A cache that stores pre-computed scores for opening game positions.
///
/// The book is stored as a `HashMap` mapping a position's unique key to its exact score.
/// A default opening book of depth 8 is embedded within the executable, providing fast
/// lookups without requiring any external files.
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
    ///
    /// This is the key function that allows an opening book to be embedded within the executable.
    pub fn from_static_bytes(bytes: &'static [u8]) -> Result<OpeningBook, bincode::Error> {
        bincode::deserialize(bytes)
    }

    /// Looks up a position's score in the opening book.
    #[inline(always)]
    pub fn get(&self, position: &Position) -> Option<i8> {
        self.map.get(&position.get_key()).copied()
    }

    /// Saves the opening book to a file using a compact binary format.
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self.map)?;
        Ok(())
    }

    /// Loads an opening book from a binary file.
    pub fn load(path: &Path) -> Result<OpeningBook, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let map = bincode::deserialize_from(reader)?;
        Ok(OpeningBook { map })
    }
}