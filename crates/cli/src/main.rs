use std::error::Error;
use std::time::{Duration, Instant};
use indicatif::{ProgressBar, ProgressStyle};
use connect_four_ai::{load_test_data, Position, Solver};

fn main() -> Result<(), Box<dyn Error>> {
    // let board_string = "\
    //     .......\
    //     ...o...\
    //     ..xx...\
    //     ..ox...\
    //     ..oox..\
    //     ..oxxo.\
    // ";
    //
    // let pos = Position::from_board_string(board_string)?;
    //
    // let moves = "444343533654";
    //
    // let pos2 = Position::from_moves(moves)?;
    //
    // println!("{:?}", pos);
    // println!("{:?}", pos2);
    //
    // println!("{:?}", compute_optimal_move(&pos));

    let pos = Position::from_moves("")?;

    let mut solver = Solver::default();

    // println!("{:?}", solver.solve(&pos));

    // return Ok(());
    test("test-data/begin-medium")?;

    Ok(())
}

fn test(path: &str) -> Result<(), Box<dyn Error>> {
    let test_positions = load_test_data(path)?;

    let mut times = Vec::with_capacity(test_positions.len());
    let mut explored_positions = Vec::with_capacity(test_positions.len());

    // Initialises progress bar
    let progress_bar_style = ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .progress_chars("#>-");
    let progress_bar = ProgressBar::new(test_positions.len() as u64).with_style(progress_bar_style);


    let mut solver = Solver::default();
    for pos in progress_bar.wrap_iter(test_positions.iter()) {
        // println!("{:?}", pos.position);
        solver.reset();

        let start_time = Instant::now();
        let best_score = solver.solve(&pos.position);
        let finish_time = Instant::now();

        assert_eq!(best_score, pos.score);
        times.push(finish_time - start_time);
        explored_positions.push(solver.explored_positions);
    }

    let total_duration = times.iter().sum::<Duration>();
    let total_explored_positions = explored_positions.iter().sum::<usize>();

    println!(
        "{}:\nMean time: {:?}, Mean no. of positions: {:.4}, kpos/s: {:.4}",
        path,
        total_duration / times.len() as u32,
        total_explored_positions as f64 / explored_positions.len() as f64,
        total_explored_positions as f64 / total_duration.as_secs_f64() / 1000.
    );

    Ok(())
}
