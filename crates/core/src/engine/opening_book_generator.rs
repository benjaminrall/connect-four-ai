//! A generator for creating a Connect Four opening book.

use crate::{OpeningBook, Position, Solver};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// Each thread gets its own static Solver instance for efficient parallelisation.
thread_local! {
    static THREAD_SOLVER: RefCell<Solver> = RefCell::new(Solver::with_opening_book());
}

/// A utility struct for generating a new `OpeningBook`.
pub struct OpeningBookGenerator;

impl OpeningBookGenerator {
    /// Generates book entries for all positions up to a given depth.
    ///
    /// # Arguments
    /// * `max_depth`: The maximum number of moves to generate positions for.
    ///
    /// # Returns
    /// A new `OpeningBook` instance containing the solved positions.
    pub fn generate(max_depth: usize) -> OpeningBook {
        let map = Arc::new(Mutex::new(HashMap::new()));

        // A set of all seen positions to avoid exploring the same position twice
        let seen = Arc::new(Mutex::new(HashSet::new()));

        // Starts the search from an initial, empty board
        let start_pos = Position::new();
        seen.lock().unwrap().insert(start_pos.get_key());
        let mut current_level = vec![start_pos];

        // Breadth-first search loop for exploring each depth in sequence
        for depth in 0..=max_depth {
            if current_level.is_empty() {
                break;
            }

            println!("Processing Depth: {}, Positions: {}", depth, current_level.len());
            let progress_bar = Self::create_progress_bar(current_level.len() as u64);

            // Solves all positions of the current depth in parallel
            let next_level_positions: Vec<Vec<Position>> = current_level
                .par_iter()
                .progress_with(progress_bar)
                .map(|pos| {
                    THREAD_SOLVER.with(|s| {
                        let mut solver = s.borrow_mut();
                        let key = pos.get_key();
                        let score = solver.solve(pos);
                        map.lock().unwrap().insert(key, score);
                        Self::generate_children(pos)
                    })
                })
                .collect();

            // Adds results to the opening book and prepares the next level of positions
            let mut next_level = Vec::new();
            let mut seen_guard = seen.lock().unwrap();
            for positions in next_level_positions {
                for pos in positions {
                    let key = pos.get_key();
                    if seen_guard.insert(key) {
                        next_level.push(pos);
                    }
                }
            }
            current_level = next_level;
        }

        let mut book = OpeningBook::new();
        book.map = Arc::try_unwrap(map)
            .expect("Error unwrapping opening book.")
            .into_inner()
            .expect("Error unwrapping opening book.");

        println!("Generation complete. Final book size: {}", book.map.len());
        book
    }


    /// Helper function to generate all possible child positions of a given position.
    fn generate_children(pos: &Position) -> Vec<Position> {
        let mut children = Vec::with_capacity(Position::WIDTH);
        let possible_moves = pos.possible();
        for col in 0..Position::WIDTH {
            if (possible_moves & Position::column_mask(col)) > 0 {
                let mut next_pos = *pos;
                next_pos.play(col);
                if !next_pos.is_won_position() {
                    children.push(next_pos);
                }
            }
        }
        children
    }

    /// Helper function to create a progress bar for tracking book generation.
    fn create_progress_bar(len: u64) -> ProgressBar {
        let progress_bar_style = ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] \
                [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})"
            )
            .unwrap()
            .progress_chars("#>-");
        ProgressBar::new(len).with_style(progress_bar_style)
    }
}