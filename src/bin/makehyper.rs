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
    io::{self, BufReader, BufWriter, Read, Write},
    path::PathBuf,
    sync::Arc,
};

/// Make Hypergammon database

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Output file
    #[arg(short = 'f', long = "file", default_value = "data/hyper.db")]
    file: PathBuf,

    /// Unique file
    #[arg(short = 'u', long = "unqiue", default_value = "data/unique.db")]
    uniquefile: PathBuf,

    /// Number of iterations
    #[arg(short = 'i', long = "iter", default_value = "100")]
    iterations: usize,

    /// Separator
    #[arg(short = 's', long = "sep", default_value = ",")]
    sep: char, // TODO: Fix this to be a single byte and accept ;

    /// Number of iterations
    #[arg(short = 'p', long = "probs")]
    probs: bool,

    /// Verbose
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

fn read_unique(args: &Args) -> io::Result<Vec<Hypergammon>> {
    let file = File::open(&args.uniquefile)?;
    let mut reader = BufReader::new(file);

    let mut buffer = [0u8; 10];
    let mut unique = Vec::new();

    while reader.read_exact(&mut buffer).is_ok() {
        let hypergammon = Hypergammon::decode(buffer);
        unique.push(hypergammon);
    }

    Ok(unique)
}

fn write_unique(args: &Args, unique: &Vec<Hypergammon>) -> io::Result<()> {
    let file = File::create(&args.uniquefile)?;
    let mut buf_writer = BufWriter::new(file);

    for position in unique.iter() {
        buf_writer.write_all(&position.encode())?;
    }

    buf_writer.flush()
}

fn write_file(args: &Args, probs: &[Probabilities]) -> io::Result<()> {
    let file = File::create(&args.file)?;
    let mut buf_writer = BufWriter::new(file);

    for prob in probs.iter() {
        if args.probs {
            buf_writer.write_all(&prob.win_normal.to_le_bytes())?;
            buf_writer.write_all(&prob.win_gammon.to_le_bytes())?;
            buf_writer.write_all(&prob.win_bg.to_le_bytes())?;
            buf_writer.write_all(&prob.lose_normal.to_le_bytes())?;
            buf_writer.write_all(&prob.lose_gammon.to_le_bytes())?;
            buf_writer.write_all(&prob.lose_bg.to_le_bytes())?;
        } else {
            buf_writer.write_all(&prob.equity().to_le_bytes())?;
        }
    }

    buf_writer.flush()
}

const POSSIBLE: usize = mcomb(26, Hypergammon::NUM_CHECKERS as usize).pow(2);
const STYLE: &str =
    "{wide_bar} {pos}/{len} ({percent}%) Elapsed: {elapsed_precise} ETA: {eta_precise}";

fn equity_update(positions: &PosMap, probs: &Vec<Probabilities>) -> Vec<Probabilities> {
    let shared_probs = Arc::new(probs);

    let style = ProgressStyle::default_bar().template(STYLE).unwrap();

    probs
        .par_iter()
        .progress_with_style(style)
        .enumerate()
        .map(|(hash, equity)| match positions.get(&hash) {
            Some(rolls) => {
                let mut possiblilies = 0.0;
                let mut total = Probabilities::empty();
                for (n, children) in rolls {
                    let best = children
                        .iter()
                        .map(|child| (shared_probs[*child], shared_probs[*child].equity()))
                        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                        .unwrap()
                        .0;
                    possiblilies += n;
                    total = Probabilities {
                        win_normal: total.win_normal + n * best.win_normal,
                        win_gammon: total.win_gammon + n * best.win_gammon,
                        win_bg: total.win_bg + n * best.win_bg,
                        lose_normal: total.lose_normal + n * best.lose_normal,
                        lose_gammon: total.lose_gammon + n * best.lose_gammon,
                        lose_bg: total.lose_bg + n * best.lose_bg,
                    }
                }
                Probabilities {
                    win_normal: total.lose_normal / possiblilies,
                    win_gammon: total.lose_gammon / possiblilies,
                    win_bg: total.lose_bg / possiblilies,
                    lose_normal: total.win_normal / possiblilies,
                    lose_gammon: total.win_gammon / possiblilies,
                    lose_bg: total.win_bg / possiblilies,
                }
            }
            None => *equity,
        })
        .collect()
}

type PosMap = HashMap<usize, Vec<(f32, Vec<usize>)>>;

fn unqiue(verbose: bool) -> Vec<Hypergammon> {
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
                    for (die, _) in ALL_21 {
                        let children = position.possible_positions(&die);
                        for child in children {
                            if !found.contains(&child) {
                                found.insert(child);
                                new_positons.push(child);
                            }
                        }
                    }
                }
                GameOver(_) => {}
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

    found.into_iter().collect()
}

fn split_positions(positions: Vec<Hypergammon>) -> (Vec<Hypergammon>, Vec<Hypergammon>) {
    let mut ongoing = vec![];
    let mut gameover = vec![];
    for position in positions {
        match position.game_state() {
            Ongoing => ongoing.push(position),
            GameOver(_) => gameover.push(position),
        }
    }
    (ongoing, gameover)
}

fn initial_equities(gameover: Vec<Hypergammon>) -> Vec<Probabilities> {
    let mut equities = vec![Probabilities::empty(); POSSIBLE];
    gameover.iter().for_each(|p| {
        equities[p.dbhash()] = Probabilities::from_result(match &p.game_state() {
            Ongoing => panic!("Should not be ongoing"),
            GameOver(result) => result,
        })
    });
    equities
}

fn create_posmap(ongoing: Vec<Hypergammon>) -> PosMap {
    let style = ProgressStyle::default_bar().template(STYLE).unwrap();

    let posmap = ongoing
        .par_iter()
        .progress_with_style(style)
        .map(|position| {
            let mut c = vec![];
            for (die, n) in ALL_21 {
                let children = position.possible_positions(&die);
                c.push((n, children.iter().map(|pos| pos.dbhash()).collect()));
            }
            (position.dbhash(), c)
        })
        .collect();

    posmap
}

fn check_open(equities: &Vec<Probabilities>) -> Probabilities {
    let position = Hypergammon::new();
    let mut possibilies = 0.0;
    let mut open = Probabilities::empty();
    for die in ALL_SINGLES {
        let children = position.possible_positions(&die);
        let best = children
            .iter()
            .map(|child| (equities[child.dbhash()], equities[child.dbhash()].equity()))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;
        possibilies += 1.0;
        open = Probabilities {
            win_normal: open.win_normal + best.win_normal,
            win_gammon: open.win_gammon + best.win_gammon,
            win_bg: open.win_bg + best.win_bg,
            lose_normal: open.lose_normal + best.lose_normal,
            lose_gammon: open.lose_gammon + best.lose_gammon,
            lose_bg: open.lose_bg + best.lose_bg,
        }
    }
    Probabilities {
        win_normal: open.lose_normal / possibilies,
        win_gammon: open.lose_gammon / possibilies,
        win_bg: open.lose_bg / possibilies,
        lose_normal: open.win_normal / possibilies,
        lose_gammon: open.win_gammon / possibilies,
        lose_bg: open.win_bg / possibilies,
    }
}

fn run(args: &Args) -> io::Result<()> {
    let positions = match read_unique(args) {
        Ok(positions) => positions,
        Err(err) => {
            println!("Error reading unique file: {}", err);
            let positions = unqiue(args.verbose);
            write_unique(args, &positions)?;
            positions
        }
    };
    println!("Positions: {}", positions.len());
    let (ongoing, gameover) = split_positions(positions);
    let mut equities = initial_equities(gameover);
    let posmap = create_posmap(ongoing);
    for iteration in 0..args.iterations {
        equities = equity_update(&posmap, &equities);
        let probs = check_open(&equities);
        println!(
            "Itr: {} Start Equity: {:.5} wn:{:.5} wg:{:.5} wb:{:.5} ln:{:.5} lg:{:.5} lb:{:.5}",
            iteration,
            probs.equity(),
            probs.win_normal + probs.win_gammon + probs.win_bg,
            probs.win_gammon + probs.win_bg,
            probs.win_bg,
            probs.lose_normal + probs.lose_gammon + probs.lose_bg,
            probs.lose_gammon + probs.lose_bg,
            probs.lose_bg,
        );
    }
    println!("Writing to {}", args.file.display());
    write_file(&args, &equities)
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    run(&args)
}
