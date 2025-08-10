//! Script to benchmark the Connect Four AI engine.
//!
//! This script evaluated the performance and accuracy of the `Solver` by running it against
//! a set of predefined test positions and their scores. The script must be run with a path
//! to a testing file as a command-line argument. Testing files are plain text files where
//! each line represents a single test case. Each line must contain two values separated by
//! a space:
//! 1. Move Sequence: A string of digits (1-7) representing the sequence of moves from the
//!    start of the game to reach the desired position.
//! 2. Expected score: The known best score for that position from the current player's
//!    perspective.

use connect_four_ai::{Position, Solver};
use indicatif::{ProgressBar, ProgressStyle};
use std::{env, fmt};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::time::{Duration, Instant};

/// Represents a single parsed test case from the input file.
pub struct TestCase {
    pub position: Position,
    pub expected_score: i8,
}

/// Implements parsing logic to convert a line of text into a `TestCase`.
impl FromStr for TestCase {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let moves = parts.next().ok_or("Missing move sequence")?;
        let score_str = parts.next().ok_or("Missing score")?;

        let position = Position::from_moves(moves)?;
        let expected_score = score_str.parse::<i8>()?;

        Ok(TestCase { position, expected_score })
    }
}

/// Stores the aggregated results from a benchmark run.
#[derive(Default)]
struct BenchmarkResults {
    total_tests: usize,
    correct_solves: usize,
    total_duration: Duration,
    total_positions_explored: usize,
    failures: Vec<(String, i8, i8)>, // (moves, expected, actual)
}

impl BenchmarkResults {
    /// Updates the results with data from a single test run.
    fn update(&mut self, moves: &str, expected: i8, actual: i8, duration: Duration, positions: usize) {
        self.total_tests += 1;
        self.total_duration += duration;
        self.total_positions_explored += positions;

        if expected == actual {
            self.correct_solves += 1;
        } else {
            self.failures.push((moves.to_string(), expected, actual));
        }
    }
}

/// Implements a clean, readable display format for the results.
impl Display for BenchmarkResults {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n--- Benchmark Results ---")?;
        writeln!(
            f,
            "Accuracy: {} / {} ({:.2}%)",
            self.correct_solves,
            self.total_tests,
            (self.correct_solves as f64 / self.total_tests as f64) * 100.0
        )?;

        if self.total_tests > 0 {
            let mean_time = self.total_duration / self.total_tests as u32;
            let mean_nodes = self.total_positions_explored as f64 / self.total_tests as f64;
            let k_pos_per_sec = self.total_positions_explored as f64 / self.total_duration.as_secs_f64() / 1000.0;

            writeln!(f, "Mean time per position: {mean_time:?}")?;
            writeln!(f, "Mean nodes explored: {mean_nodes:.0}")?;
            writeln!(f, "Solver speed: {k_pos_per_sec:.2} kpos/s")?;
        }

        if !self.failures.is_empty() {
            writeln!(f, "\n--- Failures ---")?;
            for (moves, expected, actual) in &self.failures {
                writeln!(f, "Moves: '{moves}' | Expected: {expected}, Got: {actual}")?;
            }
        }

        Ok(())
    }
}

/// Main entrypoint for the benchmark binary.
fn main() -> Result<(), Box<dyn Error>> {
    // Collects and parses command-line arguments
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Error: Missing command-line argument.");
            eprintln!("Usage: cargo run --release --bin benchmark -- <path/to/test_file>");
            return Err("No path given".into());
        }
    };

    println!("Loading test data from '{path}'...");
    let test_cases = load_test_data(&path)?;

    println!("Running benchmark on {} positions...", test_cases.len());
    let results = run_benchmark(&test_cases)?;

    // Prints the final, formatted benchmark report
    println!("{results}");

    Ok(())
}

/// Loads and parses a given test file into a vector of `TestCase` structs.
fn load_test_data(path: &str) -> Result<Vec<(String, TestCase)>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut cases = Vec::new();

    for (i, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        if line.trim().is_empty() {
            continue;
        }
        match line.parse::<TestCase>() {
            Ok(case) => cases.push((line, case)),
            Err(e) => return Err(format!("Error parsing line {}: {}", i + 1, e).into()),
        }
    }

    Ok(cases)
}

/// Runs a Connect Four solver against all test cases and aggregates the results.
fn run_benchmark(test_cases: &[(String, TestCase)]) -> Result<BenchmarkResults, Box<dyn Error>> {
    let mut results = BenchmarkResults::default();
    let mut solver = Solver::new();

    let progress_bar = create_progress_bar(test_cases.len() as u64);

    for (line_str, test_case) in progress_bar.wrap_iter(test_cases.iter()) {
        solver.reset();

        let start_time = Instant::now();
        let actual_score = solver.solve(&test_case.position);
        let duration = start_time.elapsed();

        results.update(line_str, test_case.expected_score, actual_score, duration, solver.explored_positions);
    }

    Ok(results)
}

/// Helper function to create a styled progress bar.
fn create_progress_bar(len: u64) -> ProgressBar {
    let style = ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})"
        )
        .unwrap()
        .progress_chars("#>-");
    ProgressBar::new(len).with_style(style)
}