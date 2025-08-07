use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Position;

pub struct TestPosition {
    pub position: Position,
    pub score: i8,
}

pub fn load_test_data(path: &str) -> Result<Vec<TestPosition>, Box<dyn Error>> {
    let file = BufReader::new(File::open(path)?);

    let mut test_positions = Vec::with_capacity(1000);
    for line in file.split(b'\n') {
        let buf = String::from_utf8(line?)?;
        let mut test_data = buf.split_whitespace();
        let moves = test_data.next().expect("Failed to read test data");
        let score = test_data.next().expect("Failed to read test data").parse::<i8>()?;
        let position = Position::from_moves(moves)?;
        test_positions.push(TestPosition { position, score });
    }

    Ok(test_positions)
}