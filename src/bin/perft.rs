use bkgm::{
    dice::ALL_21,
    GameState::{GameOver, Ongoing},
    Position,
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

fn perft_rec(depth: usize, position: &Position) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut count = 0;
    for (die, _) in ALL_21 {
        let children = position.all_positions_after_moving(&die);
        for child in children {
            count += match child.game_state() {
                Ongoing => perft_rec(depth - 1, &child),
                GameOver(_) => 1,
            };
        }
    }
    count
}

fn perft(args: &Args) {
    let position = Position::from_id(&args.position).expect("Invalid position");
    let mut total = 0;
    for (die, _) in ALL_21 {
        let mut count = 0;
        let children = position.all_positions_after_moving(&die);
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
    println!("Total: {}", total);
}

fn main() {
    let args = Args::parse();
    perft(&args);
}
