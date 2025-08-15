//! A script to analyse and tune the AI player's difficulty settings.
//!
//! The script must be run with a position's move sequence string specified as the first
//! command-line argument - if not specified, an empty position will be used.

use connect_four_ai::{AIPlayer, Difficulty, Position};
use std::collections::HashMap;
use std::env;
use std::error::Error;

const SIMULATION_COUNT: u32 = 10000;

fn main() -> Result<(), Box<dyn Error>> {
    // Collects and parses command-line arguments
    let args: Vec<String> = env::args().collect();
    let pos_moves = args.get(1)
        .map_or("", |s| s);

    println!(
        "Running {} simulations for position: '{}'...",
        SIMULATION_COUNT,
        if pos_moves.is_empty() { "Empty Position" } else { pos_moves }
    );

    let difficulties = [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Impossible,
    ];
    let mut players: Vec<_> = difficulties.iter().map(|&d| AIPlayer::new(d)).collect();
    let position = Position::from_moves(pos_moves)?;

    // Determines the move scores for the position, and calculates the optimal score from them
    let move_scores = players.last_mut().unwrap().get_all_move_scores(&position);
    let optimal_score = move_scores
        .iter()
        .filter_map(|&s| s)
        .max()
        .unwrap_or(0);

    // Sets up lists to hold the results
    let mut choice_counts: Vec<HashMap<usize, u32>> = vec![HashMap::new(); players.len()];
    let mut optimal_counts = vec![0; players.len()];
    let mut winning_counts = vec![0; players.len()];
    let mut drawing_counts = vec![0; players.len()];
    let mut losing_counts = vec![0; players.len()];

    for _ in 0..SIMULATION_COUNT {
        for (i, player) in players.iter_mut().enumerate() {
            if let Some(choice) = player.select_move(&position, &move_scores) {
                // Record which column was chosen.
                *choice_counts[i].entry(choice).or_insert(0) += 1;

                // Check if the choice was optimal, winning, drawing, or losing.
                if let Some(chosen_score) = move_scores[choice] {
                    if chosen_score == optimal_score {
                        // An optimal move is the best possible move in the position
                        optimal_counts[i] += 1;
                    }
                    if chosen_score > 0 {
                        // A winning move is any positive move resulting in a win for the player
                        winning_counts[i] += 1;
                    }
                    if chosen_score >= 0 {
                        // A "drawing move" is any winning move or drawing move
                        drawing_counts[i] += 1;
                    } else {
                        // A losing move is any other move, which allows the opponent to win
                        losing_counts[i] += 1;
                    }
                }
            }
        }
    }

    println!("\n--- Simulation Results ---");
    println!("Move scores for this position: {move_scores:?}");
    println!("Optimal score for this position: {optimal_score}");

    // Displays the results for each difficulty
    for (i, &difficulty) in difficulties.iter().enumerate() {
        println!("\n--- Difficulty: {difficulty:?} ---");
        println!("Move Distribution:");

        let mut sorted_choices: Vec<_> = choice_counts[i].iter().collect();
        sorted_choices.sort_by_key(|a| a.0);

        for (col, count) in sorted_choices {
            let percentage = (*count as f64 / SIMULATION_COUNT as f64) * 100.0;
            println!("  Col {col}: {count:>5} times ({percentage:>5.2}%)");
        }

        let optimal_percentage = (optimal_counts[i] as f64 / SIMULATION_COUNT as f64) * 100.0;
        println!(
            "Optimal Moves: {:>5.2}% ({}/{} choices)",
            optimal_percentage, optimal_counts[i], SIMULATION_COUNT
        );

        let winning_percentage = (winning_counts[i] as f64 / SIMULATION_COUNT as f64) * 100.0;
        println!(
            "Winning Moves: {:>5.2}% ({}/{} choices)",
            winning_percentage, winning_counts[i], SIMULATION_COUNT
        );

        let drawing_percentage = (drawing_counts[i] as f64 / SIMULATION_COUNT as f64) * 100.0;
        println!(
            "Drawing Moves: {:>5.2}% ({}/{} choices)",
            drawing_percentage, drawing_counts[i], SIMULATION_COUNT
        );

        let losing_percentage = (losing_counts[i] as f64 / SIMULATION_COUNT as f64) * 100.0;
        println!(
            "Losing Moves:  {:>5.2}% ({}/{} choices)",
            losing_percentage, losing_counts[i], SIMULATION_COUNT
        );
    }

    Ok(())
}
