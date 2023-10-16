use bkgm::{
    dice::{ALL_21, ALL_SINGLES},
    utils::mcomb,
    GameState::{GameOver, Ongoing},
    Hypergammon, State,
};
use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
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

    /// Number of iterations
    #[arg(short = 'i', long = "iter", default_value = "100")]
    iterations: usize,

    /// Number of checkers
    #[arg(short = 'c', long = "checkers", default_value = "3")]
    checkers: usize,

    /// Resume
    #[arg(short = 'r', long = "resume")]
    resume: Option<PathBuf>,

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

fn equity_update(positions: &Unique, equities: &Vec<f32>) -> Vec<f32> {
    let style = ProgressStyle::default_bar()
        .template(
            "{wide_bar} {pos}/{len} ({percent}%) Elapsed: {elapsed_precise} ETA: {eta_precise}",
        )
        .unwrap();

    equities
        .par_iter()
        .progress_with_style(style)
        .enumerate()
        .map(|(h, e)| match positions.get(&h) {
            Some(rolls) => {
                let mut possiblilies = 0.0;
                let mut total = 0.0;
                for (n, children) in rolls {
                    let equity = children
                        .iter()
                        .map(|pos| equities[*pos])
                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap();
                    possiblilies += n;
                    total += n * -equity;
                }
                total / possiblilies
            }
            None => *e,
        })
        .collect()
}

type Unique = HashMap<usize, Vec<(f32, Vec<usize>)>>;

fn unqiue(verbose: bool) -> (Unique, Vec<f32>) {
    let mut equities = vec![0.0; POSSIBLE];
    let mut non_terminal = HashMap::with_capacity(POSSIBLE);
    let position = Hypergammon::new();
    let mut found = HashSet::new();
    let mut new_positons = vec![];
    let before = found.len();

    for die in ALL_SINGLES {
        let children = position.possible_positions(&die);
        for child in children {
            if !found.contains(&child) {
                found.insert(child);
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
            match position.game_state() {
                Ongoing => {
                    let mut c = vec![];
                    for (die, n) in ALL_21 {
                        let children = position.possible_positions(&die);
                        c.push((n, children.iter().map(|pos| pos.dbhash()).collect()));
                        for child in children {
                            if !found.contains(&child) {
                                found.insert(child);
                                new_positons.push(child);
                            }
                        }
                    }
                    non_terminal.insert(position.dbhash(), c);
                }
                GameOver(result) => {
                    equities[position.dbhash()] = result.value();
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

    (non_terminal, equities)
}

fn run(args: &Args) -> io::Result<()> {
    let (positions, initial) = unqiue(args.verbose);
    let mut equities = initial;
    let starting = Hypergammon::new().dbhash();
    println!("Positions: {}", positions.len());
    for iteration in 0..args.iterations {
        equities = equity_update(&positions, &equities);
        println!(
            "Iter: {}\tStart Equity: {:.5}",
            iteration + 1,
            equities[starting]
        );
    }
    println!("Writing to {}", args.file.display());
    write_file(&args.file, &equities)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run(&args)
}
