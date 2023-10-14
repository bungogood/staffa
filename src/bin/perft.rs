use bkgm::{
    dice::{ALL_21, ALL_SINGLES},
    Backgammon,
    GameState::{GameOver, Ongoing},
    State,
};
use clap::Parser;

/// Benchmark and test position / move generation

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of games to generate
    #[arg(short = 'd', long = "depth")]
    depth: usize,

    /// Position
    #[arg(short = 'p', long = "position", default_value = "4HPwATDgc/ABMA")]
    position: String,

    /// Verbose
    #[arg(short = 'v', long = "verbose", default_value = "false")]
    verbose: bool,
}

fn perft_rec(depth: usize, position: &Backgammon) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut count = 0;
    for (die, _) in ALL_21 {
        let children = position.possible_positions(&die);
        for child in children {
            count += match child.game_state() {
                Ongoing => perft_rec(depth - 1, &child),
                GameOver(_) => 1,
            };
        }
    }
    count
}

fn perft(args: &Args) -> u64 {
    let position = Backgammon::from_id(&args.position).expect("Invalid position");
    if args.verbose {
        position.show();
    }
    let mut total = 0;
    for die in ALL_SINGLES {
        let mut count = 0;
        let children = position.possible_positions(&die);
        for child in children {
            count += match child.game_state() {
                Ongoing => perft_rec(args.depth - 1, &child),
                GameOver(_) => 1,
            };
        }
        if args.verbose {
            println!("- {}: {}", die, count);
        }
        total += count;
    }
    total
}

fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();
    let total = perft(&args);
    let dur = start.elapsed();
    let speed = total as f64 / dur.as_secs_f64();
    let avg_time = dur / total as u32;
    println!("Total: {}", total);
    println!(
        "Elapsed: {:.2?} Speed: {:.2}/s, Avg: {:.2?}",
        dur, speed, avg_time
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perft() {
        let args = Args {
            depth: 2,
            position: "4HPwATDgc/ABMA".to_string(),
            verbose: false,
        };
        let total = perft(&args);
        assert_eq!(total, 447);
    }
}
