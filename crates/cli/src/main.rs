use connect_four_ai::{load_test_data, OpeningBook, Position, Solver};
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::path::Path;
use std::time::{Duration, Instant};

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
    // let pos = Position::from_moves("274121776146")?;
    // println!("{:?}", pos.get_key());
    // return Ok(());
    // let actual_score = 4;
    // let mut solver = Solver::default();
    // assert_eq!(actual_score, solver.solve(&pos));

    // let pos = Position::new();
    // println!("{:?}", Solver::new().solve(&pos));
    // Ok(())

    // test("test-data/begin-hard")?;
    // return Ok(());

    let path = Path::new("book.bin");
    let mut book = OpeningBook::load(path).unwrap_or(OpeningBook::new());
    // A depth of 8-10 is a good starting point.
    // Be aware that generation time increases exponentially.
    let max_depth = 7;
    for depth in 0..=max_depth {
        println!("Generating opening book to depth {}...", depth);
        book.generate(depth);
        println!("Generation complete. Found {} positions.", book.map.len());

        println!("Saving to {:?}...", path);
        book.save(path).expect("Failed to save book file.");
        println!("Done.");
    }
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


    let mut solver = Solver::new();
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
