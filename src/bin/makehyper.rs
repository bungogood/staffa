use bkgm::{
    dice::{ALL_21, ALL_SINGLES},
    utils::mcomb,
    GameState::{GameOver, Ongoing},
    Hypergammon, State,
};
use clap::Parser;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
};

/// Make Hypergammon database

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file
    #[arg(short = 'f', long = "file", default_value = "data/hyper.db")]
    file: PathBuf,

    /// Number of checkers
    #[arg(short = 'c', long = "checkers", default_value = "3")]
    checkers: usize,

    /// Verbose
    #[arg(short = 'v', long = "verbose", default_value = "false")]
    verbose: bool,
}

fn write_file(file: &PathBuf, data: &[f32]) -> io::Result<()> {
    // Open a binary file for writing
    let file = File::create(file)?;

    let mut buf_writer = BufWriter::new(file);

    // Write the array to the file as binary data
    for &value in data.iter() {
        buf_writer.write_all(&value.to_le_bytes())?;
    }

    buf_writer.flush()
}

const POSSIBLE: usize = mcomb(26, Hypergammon::NUM_CHECKERS as usize).pow(2);

// fn equity_update(positions: &HashSet<Hypergammon>, equities: Vec<f32>) -> Vec<f32> {
//     let mut updated = equities.clone();

//     let mut delta = 0.0;
//     for position in positions {
//         let equity = match position.game_state() {
//             Ongoing => {
//                 let mut total = 0.0;
//                 for (die, n) in ALL_21 {
//                     let children = position.possible_positions(&die);
//                     total += children
//                         .iter()
//                         .map(|pos| equities[pos.dbhash()])
//                         .min_by(|a, b| a.partial_cmp(b).unwrap())
//                         .unwrap()
//                         * n;
//                 }
//                 total / 36.0
//             }
//             GameOver(result) => result.value(),
//         };
//         updated[position.dbhash()] = equity;
//         delta += (equity - equities[position.dbhash()]).abs();
//     }
//     println!("Delta: {}", delta);
//     updated
// }

// fn calculate_equities(positions: &HashSet<Hypergammon>) -> Vec<f32> {
//     let mut equities: Vec<f32> = vec![0.0; POSSIBLE];
//     println!("Equity: {}", equities.len());

//     for &position in positions {
//         let equity = match position.game_state() {
//             Ongoing => 0.0,
//             GameOver(result) => result.value(),
//         };
//         equities[position.dbhash()] = equity;
//     }

//     println!("Positions: {}", equities.len());
//     for _ in 0..200 {
//         equities = equity_update(positions, equities);
//     }
//     println!("Equity: {}", equities.len());

//     equities
// }

fn equity_update(equities: &HashMap<Hypergammon, f32>) -> HashMap<Hypergammon, f32> {
    let mut updated = equities.clone();

    let mut delta = 0.0;
    for position in equities.keys() {
        let equity = match position.game_state() {
            Ongoing => {
                let mut total = 0.0;
                for (die, n) in ALL_21 {
                    let children = position.possible_positions(&die);
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

fn calculate_equities(positions: &HashSet<Hypergammon>) -> HashMap<Hypergammon, f32> {
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

fn unqiue(verbose: bool) -> HashSet<Hypergammon> {
    let position = Hypergammon::new();
    let mut found = HashSet::new();
    let mut new_positons = vec![];
    let before = found.len();
    for die in ALL_SINGLES {
        let children = position.possible_positions(&die);
        for child in children {
            if !found.contains(&child) {
                found.insert(child.clone());
                new_positons.push(child);
            }
        }
    }

    let mut depth = 1;
    let discovered = found.len() - before;
    if verbose {
        println!(
            "{}\t{}\tpositions reached after {} roll",
            discovered,
            found.len(),
            depth
        );
    }

    while !new_positons.is_empty() {
        let mut queue = new_positons;
        new_positons = vec![];
        let before = found.len();
        while let Some(position) = queue.pop() {
            for (die, _) in ALL_21 {
                let children = position.possible_positions(&die);
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
        if verbose {
            println!(
                "{}\t{}\tpositions reached after {} rolls",
                discovered,
                found.len(),
                depth
            );
        }
    }

    found
}

fn run(args: &Args) -> io::Result<()> {
    let positions = unqiue(args.verbose);
    let equities = calculate_equities(&positions);
    println!("Writing to {}", args.file.display());
    // write_file(&args.file, &equities)
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run(&args)
}
