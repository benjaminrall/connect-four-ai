use std::error::Error;
use std::path::Path;
use connect_four_ai::{AIPlayer, Difficulty, Position};

fn main() -> Result<(), Box<dyn Error>> {
    let mut position = Position::new();
    let mut player = AIPlayer::new(Difficulty::Impossible);

    while !position.is_won_position() {
        let moves = player.get_all_move_scores(&position);
        let choice = player.select_move(&position, &moves).unwrap();

        println!("{:?} {:?} {}", position, moves, choice);

        position.play(choice);
    }

    let x: u64 = (1 << 23) + 9;
    let y: usize = (1 << 23) + 9;
    let z: u32 = (1 << 23) + 9;

    println!("{x} {y} {z}");

    Ok(())
}