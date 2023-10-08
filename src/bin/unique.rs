use std::collections::{HashMap, HashSet};

use bkgm::{
    dice::{ALL_21, ALL_SINGLES},
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

struct Roll {
    children: Vec<Position>,
    prior: f32,
}

fn equity_update(equities: &HashMap<Position, f32>) -> HashMap<Position, f32> {
    let mut updated = equities.clone();

    let mut delta = 0.0;
    for position in equities.keys() {
        let equity = match position.game_state() {
            Ongoing => {
                let mut total = 0.0;
                for (die, n) in ALL_21 {
                    let children = position.all_positions_after_moving(&die);
                    total += children
                        .iter()
                        .map(|pos| equities[pos])
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap()
                        * n;
                }
                total / 36.0
            }
            GameOver(result) => result.value(),
        };
        updated.insert(position.clone(), equity);
        delta += (equity - equities[position]).abs();
    }
    println!("Delta: {}", delta);
    updated
}

fn equity(positions: &HashSet<Position>) -> HashMap<Position, f32> {
    let mut equities = HashMap::new();
    for &position in positions {
        let equity = match position.game_state() {
            Ongoing => 0.0,
            GameOver(result) => result.value(),
        };
        equities.insert(position, equity);
    }

    println!("Positions: {}", equities.len());
    for _ in 0..200 {
        equities = equity_update(&equities);
    }
    println!("Equity: {}", equities.len());

    equities
}

fn unqiue(args: &Args) -> HashSet<Position> {
    // let position = Position::from_id(&args.position).expect("Invalid position");
    let position = Position::hypergammon();
    let mut found = HashSet::new();
    let mut new_positons = vec![];
    let before = found.len();
    for die in ALL_SINGLES {
        let children = position.all_positions_after_moving(&die);
        for child in children {
            if !found.contains(&child) {
                found.insert(child.clone());
                new_positons.push(child);
            }
        }
    }

    let mut depth = 1;
    let discovered = found.len() - before;
    println!(
        "{}\t{}\tpositions reached after {} roll",
        discovered,
        found.len(),
        depth
    );

    while !new_positons.is_empty() {
        let mut queue = new_positons;
        new_positons = vec![];
        let before = found.len();
        while let Some(position) = queue.pop() {
            for (die, _) in ALL_21 {
                let children = position.all_positions_after_moving(&die);
                for child in children {
                    if !found.contains(&child) {
                        found.insert(child.clone());
                        new_positons.push(child);
                    }
                }
            }
        }
        let discovered = found.len() - before;
        depth += 1;
        println!(
            "{}\t{}\tpositions reached after {} rolls",
            discovered,
            found.len(),
            depth
        );
    }

    found
}

fn main() {
    let args = Args::parse();
    let start = std::time::Instant::now();
    let unique = unqiue(&args);
    equity(&unique);
    let total = unique.len();
    let dur = start.elapsed();
    let speed = total as f64 / dur.as_secs_f64();
    let avg_time = dur / total as u32;
    println!(
        "Elapsed: {:.2?}, Positions: {} Speed: {:.2}/s, Avg: {:.2?}",
        dur, total, speed, avg_time
    );
}
