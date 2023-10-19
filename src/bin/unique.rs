use bkgm::{
    dice::{ALL_21, ALL_SINGLES},
    position::GamePhase,
    GameResult,
    GameState::{GameOver, Ongoing},
    Hypergammon, State,
};
use clap::Parser;
use std::{collections::HashMap, fs::File, io, path::PathBuf};

/// Make Hypergammon database

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file
    #[arg(short = 'f', long = "file", default_value = "data/unique.csv")]
    file: PathBuf,

    /// Separator
    #[arg(short = 's', long = "sep", default_value = ",")]
    sep: char, // TODO: Fix this to be a single byte and accept ;

    /// Verbose
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn write_file(args: &Args, positions: &HashMap<Hypergammon, u8>) -> io::Result<()> {
    // Open a binary file for writing
    let file = File::create(&args.file)?;

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(args.sep as u8)
        .from_writer(file);

    wtr.write_record(["positionid", "depth", "phase"])?;

    let mut ordered_positions = positions
        .iter()
        .map(|(pos, depth)| (pos, depth))
        .collect::<Vec<_>>();

    ordered_positions.sort_by(|a, b| a.1.cmp(b.1));

    for (pos, depth) in ordered_positions {
        let phase = match pos.phase() {
            GamePhase::Ongoing(phase) => format!("{:?}", phase),
            GamePhase::GameOver(GameResult::LoseNormal) => "Normal".to_string(),
            GamePhase::GameOver(GameResult::LoseGammon) => "Gammon".to_string(),
            GamePhase::GameOver(GameResult::LoseBackgammon) => "Backgammon".to_string(),
            GamePhase::GameOver(_) => unreachable!(),
        };
        wtr.write_record([pos.position_id(), depth.to_string(), phase])?;
    }
    wtr.flush()
}

fn unqiue(verbose: bool) -> HashMap<Hypergammon, u8> {
    let position = Hypergammon::new();
    let mut found = HashMap::new();
    let mut new_positons = vec![];
    let before = found.len();

    let mut depth = 1;

    for die in ALL_SINGLES {
        let children = position.possible_positions(&die);
        for child in children {
            if !found.contains_key(&child) {
                found.insert(child, depth);
                new_positons.push(child);
            }
        }
    }

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
        depth += 1;
        let mut queue = new_positons;
        new_positons = vec![];
        let before = found.len();
        while let Some(position) = queue.pop() {
            match position.game_state() {
                Ongoing => {
                    for (die, _) in ALL_21 {
                        let children = position.possible_positions(&die);
                        for child in children {
                            if !found.contains_key(&child) {
                                found.insert(child, depth);
                                new_positons.push(child);
                            }
                        }
                    }
                }
                GameOver(_) => {}
            }
        }
        let discovered = found.len() - before;
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
    println!("Writing to {}", args.file.display());
    write_file(args, &positions)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run(&args)
}
