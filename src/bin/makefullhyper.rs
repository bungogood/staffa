use bkgm::{
    dice::{ALL_21, ALL_SINGLES},
    utils::mcomb,
    GameState::{GameOver, Ongoing},
    Hypergammon, State,
};
use clap::Parser;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use staffa::probabilities::Probabilities;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
    sync::Arc,
};

/// Make Hypergammon database

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file
    #[arg(short = 'f', long = "file", default_value = "data/hyper.full.db")]
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

fn write_file(file: &PathBuf, probs: &[Probabilities]) -> io::Result<()> {
    // Open a binary file for writing
    let file = File::create(file)?;

    let mut buf_writer = BufWriter::new(file);

    // Write the array to the file as binary data
    for &prob in probs.iter() {
        buf_writer.write_all(&prob.win_normal.to_le_bytes())?;
        buf_writer.write_all(&prob.win_gammon.to_le_bytes())?;
        buf_writer.write_all(&prob.win_bg.to_le_bytes())?;
        buf_writer.write_all(&prob.lose_normal.to_le_bytes())?;
        buf_writer.write_all(&prob.lose_gammon.to_le_bytes())?;
        buf_writer.write_all(&prob.lose_bg.to_le_bytes())?;
    }

    buf_writer.flush()
}

const POSSIBLE: usize = mcomb(26, Hypergammon::NUM_CHECKERS as usize).pow(2);

fn equity_update(positions: &Unique, probs: &Vec<Probabilities>) -> Vec<Probabilities> {
    let shared_probs = Arc::new(probs);

    let style = ProgressStyle::default_bar()
        .template(
            "{wide_bar} {pos}/{len} ({percent}%) Elapsed: {elapsed_precise} ETA: {eta_precise}",
        )
        .unwrap();

    probs
        .par_iter()
        .progress_with_style(style)
        .enumerate()
        .map(|(h, e)| match positions.get(&h) {
            Some(rolls) => {
                let mut possiblilies = 0.0;
                let mut total = Probabilities::empty();
                for (n, children) in rolls {
                    let equity = children
                        .iter()
                        .map(|pos| (shared_probs[*pos], shared_probs[*pos].equity()))
                        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                        .unwrap()
                        .0;
                    possiblilies += n;
                    total = Probabilities {
                        win_normal: total.win_normal + n * equity.win_normal,
                        win_gammon: total.win_gammon + n * equity.win_gammon,
                        win_bg: total.win_bg + n * equity.win_bg,
                        lose_normal: total.lose_normal + n * equity.lose_normal,
                        lose_gammon: total.lose_gammon + n * equity.lose_gammon,
                        lose_bg: total.lose_bg + n * equity.lose_bg,
                    }
                }
                Probabilities {
                    win_normal: total.win_normal / possiblilies,
                    win_gammon: total.win_gammon / possiblilies,
                    win_bg: total.win_bg / possiblilies,
                    lose_normal: total.lose_normal / possiblilies,
                    lose_gammon: total.lose_gammon / possiblilies,
                    lose_bg: total.lose_bg / possiblilies,
                }
            }
            None => *e,
        })
        .collect()
}

type Unique = HashMap<usize, Vec<(f32, Vec<usize>)>>;

fn unqiue(verbose: bool) -> (Unique, Vec<Probabilities>) {
    let mut equities = vec![Probabilities::empty(); POSSIBLE];
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
                    equities[position.dbhash()] = Probabilities::from_result(&result);
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
            "Itr: {}\tStart Equity: wn:{:.5} wg:{:.5} wb:{:.5} ln:{:.5} lg:{:.5} lb:{:.5}",
            iteration + 1,
            equities[starting].win_normal,
            equities[starting].win_gammon,
            equities[starting].win_bg,
            equities[starting].lose_normal,
            equities[starting].lose_gammon,
            equities[starting].lose_bg,
        );
    }
    println!("Writing to {}", args.file.display());
    write_file(&args.file, &equities)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run(&args)
}
