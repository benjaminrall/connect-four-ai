//! Script to generate an opening book for the Connect Four AI engine.
//!
//! The script must be run with the depth specified as the first command-line argument,
//! and an optional path to save the opening book to (default: book.bin).

use std::env;
use std::error::Error;
use std::path::Path;
use connect_four_ai::OpeningBookGenerator;

fn main() -> Result<(), Box<dyn Error>> {
    // Collects and parses command-line arguments
    let args: Vec<String> = env::args().collect();
    let depth = args.get(1)
        .and_then(|arg| arg.parse().ok())
        .expect("Please specify a depth as the first command line argument.");
    let path = Path::new(args.get(2)
        .map_or("book.bin", |s| s));

    // Generates and saves the book
    println!("Generating opening book to depth {}...", depth);
    let book = OpeningBookGenerator::generate(depth);
    println!("Saving book with {} positions to {:?}...", book.map.len(), path);
    book.save(path).expect("Failed to save opening book.");
    println!("Successfully saved opening book.");

    Ok(())
}